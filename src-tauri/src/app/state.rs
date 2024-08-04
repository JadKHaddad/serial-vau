use std::{collections::HashMap, ops::Deref, sync::Arc};

#[cfg(feature = "subscriptions")]
use open_serial_port::TxHandle;
use open_serial_port::{OpenSerialPort, OutgoingPacket, SendError};
use parking_lot::RwLock;

use crate::serial::AvailablePortsError;

use super::model::managed_serial_port::{ManagedSerialPort, Status};

pub mod open_serial_port;

#[derive(Debug, Clone, Default)]
pub struct AppState {
    inner: Arc<AppStateInner>,
}

impl Deref for AppState {
    type Target = AppStateInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[derive(Debug, Default)]
pub struct AppStateInner {
    /// - `Key`: Serial port name.
    /// - `Value`: Open serial port [`OpenSerialPort`].
    ///
    /// Not using an async `RwLock` because [`WMIConnection`](wmi::WMIConnection) is not [`Send`],
    /// which is used in [`Watcher`](crate::serial::watcher::Watcher),
    /// which is used in [`run`](crate::app::run).
    open_serial_ports: RwLock<HashMap<String, OpenSerialPort>>,
    /// - `Key`: Master Serial port name.
    /// - `Value`:  
    ///     - `Key`: Subscriber serial port name.
    ///     - `Value`: Optional subscriber's [`TxHandle`] to send data to the subscriber.
    ///        - `Some(TxHandle)`: Subscriber is open.
    ///        - `None`: Subscriber is closed.
    ///
    /// Closing the Subscriber serial port will set its value to `None`. The subscription will not be removed.
    ///
    /// ## Notes
    ///
    /// - Subscriptions can exist before the master or subscriber is open.
    /// - Subscriptions can be self-referential.
    /// - Subscriptions can exist even if the `name` of a serial port does not exist (yet).
    /// - Subscriptions are not removed when the master or subscriber is closed or removed from system.
    /// - Subscriptions are removed manually.
    #[cfg(feature = "subscriptions")]
    subscriptions: Arc<RwLock<HashMap<String, HashMap<String, Option<TxHandle>>>>>,
}

impl AppStateInner {
    pub fn managed_serial_ports(&self) -> Result<Vec<ManagedSerialPort>, ManagedSerialPortsError> {
        let available_serial_ports = crate::serial::available_ports()?;
        let open_serial_ports = self.open_serial_ports.read();
        #[cfg(feature = "subscriptions")]
        let subscriptions = self.subscriptions.read();

        let managed_serial_ports = available_serial_ports
            .into_iter()
            .map(|port| {
                #[cfg(feature = "subscriptions")]
                let subscribed_to = subscriptions
                    .iter()
                    .filter(|&(_, tx_handles)| tx_handles.contains_key(port.name()))
                    .map(|(name, _)| name.clone())
                    .collect::<Vec<_>>();

                #[cfg(feature = "subscriptions")]
                let subscriptions = subscriptions
                    .get(port.name())
                    .unwrap_or(&HashMap::new())
                    .iter()
                    .map(|(name, _)| name.clone())
                    .collect();

                let mut managed_serial_port = ManagedSerialPort {
                    name: port.name().to_string(),
                    status: Status::Closed,
                    #[cfg(feature = "subscriptions")]
                    subscriptions,
                    #[cfg(feature = "subscriptions")]
                    subscribed_to,
                    read_state: None,
                };

                if let Some(open_serial_port) = open_serial_ports.get(port.name()) {
                    managed_serial_port.status = Status::Open;
                    managed_serial_port.read_state = Some(open_serial_port.read_state());
                }

                managed_serial_port
            })
            .collect::<Vec<_>>();

        Ok(managed_serial_ports)
    }

    /// Adds the serial port to [`Self::open_serial_ports`] and adds it to all subscriptions.
    pub fn add_open_serial_port(&self, open_serial_port: OpenSerialPort) -> Option<OpenSerialPort> {
        tracing::debug!(name=%open_serial_port.name(), "Adding serial port");

        #[cfg(feature = "subscriptions")]
        self.add_open_serial_port_to_pending_subscriptions(&open_serial_port);

        self.open_serial_ports
            .write()
            .insert(open_serial_port.name().to_string(), open_serial_port)
    }

    /// Sets the [`Option<TxHandle>`] of the given serial port to `Some(TxHandle)` in all subscriptions.
    ///
    /// A subscription can exist before the subscriber is open.
    #[cfg(feature = "subscriptions")]
    fn add_open_serial_port_to_pending_subscriptions(&self, open_serial_port: &OpenSerialPort) {
        tracing::debug!(name=%open_serial_port.name(), "Adding serial port to pending subscriptions");

        let mut subscriptions = self.subscriptions.write();

        for (_, tx_handles) in subscriptions.iter_mut() {
            tx_handles
                .get_mut(open_serial_port.name())
                .map(|tx_handle| tx_handle.replace(open_serial_port.tx_handle()));
        }
    }

    /// Cancels the subscription of the given serial port in all subscriptions.
    ///
    /// Sets the [`Option<TxHandle>`] of the given serial port to `None` in all subscriptions.
    ///
    /// The subscription will not be removed.
    #[cfg(feature = "subscriptions")]
    fn remove_open_serial_port_from_all_subscriptions(&self, name: &str) {
        tracing::debug!(name=%name, "Removing serial port as subscriber from all subscriptions");

        let mut subscriptions = self.subscriptions.write();

        for (_, tx_handles) in subscriptions.iter_mut() {
            tx_handles.get_mut(name).map(|tx_handle| tx_handle.take());
        }
    }

    /// Removes the serial port from [`Self::open_serial_ports`] and cancels its subscription.
    pub fn remove_open_serial_port(&self, name: &str) -> Option<OpenSerialPort> {
        tracing::debug!(name=%name, "Removing serial port");

        #[cfg(feature = "subscriptions")]
        self.remove_open_serial_port_from_all_subscriptions(name);
        self.open_serial_ports.write().remove(name)
    }

    /// Removes and cancels the serial port from [`Self::open_serial_ports`] and cancels its subscription.
    pub fn remove_and_cancel_open_serial_port(&self, name: &str) -> Option<OpenSerialPort> {
        self.remove_open_serial_port(name)
            .map(OpenSerialPort::cancelled)
    }

    /// - `Ok(Some(bool))` => Port found
    /// - `Ok(None)` => Port not found
    pub fn is_port_open(&self, name: &str) -> Result<Option<bool>, ManagedSerialPortsError> {
        let managed_serial_ports = self.managed_serial_ports()?;
        let managed_serial_port = managed_serial_ports.iter().find(|port| port.name == name);

        Ok(managed_serial_port.map(|port| port.is_open()))
    }

    /// - `Ok(Some(bool))` => Port found
    /// - `Ok(None)` => Port not found
    pub fn is_port_closed(&self, name: &str) -> Result<Option<bool>, ManagedSerialPortsError> {
        let managed_serial_ports = self.managed_serial_ports()?;
        let managed_serial_port = managed_serial_ports.iter().find(|port| port.name == name);

        Ok(managed_serial_port.map(|port| port.is_closed()))
    }

    /// - `Some(Ok())` => Ok
    /// - `Some(Err(_))` => Send error
    /// - `None` => Port not found
    pub fn send_to_open_serial_port(
        &self,
        name: &str,
        packet: OutgoingPacket,
    ) -> Option<Result<(), SendError>> {
        Some(self.open_serial_ports.read().get(name)?.send(packet))
    }

    pub fn send_to_all_open_serial_ports(&self, packet: OutgoingPacket) {
        self.open_serial_ports.read().values().for_each(|port| {
            // Cheap clone
            let _ = port.send(packet.clone());
        })
    }

    #[cfg(feature = "subscriptions")]
    pub fn subscriptions(&self) -> Arc<RwLock<HashMap<String, HashMap<String, Option<TxHandle>>>>> {
        self.subscriptions.clone()
    }

    /// `to` is subscribed to `from`.
    ///
    /// - `from` will send data to `to`.
    /// - `to` will receive data from `from`.
    #[cfg(feature = "subscriptions")]
    pub fn subscribe(&self, from: &str, to: &str) {
        tracing::debug!(%from, %to, "Subscribing");

        let mut subscriptions = self.subscriptions.write();

        let tx_handle = self
            .open_serial_ports
            .read()
            .get(to)
            .map(|port| port.tx_handle());

        subscriptions
            .entry(from.to_string())
            .or_default()
            .insert(to.to_string(), tx_handle);
    }

    /// `to` is unsubscribed from `from`.
    ///
    /// - `from` will no longer send data to `to`.
    /// - `to` will no longer receive data from `from`.,
    #[cfg(feature = "subscriptions")]
    pub fn unsubscribe(&self, from: &str, to: &str) {
        tracing::debug!(%from, %to, "Unsubscribing");

        let mut subscriptions = self.subscriptions.write();

        subscriptions
            .get_mut(from)
            .and_then(|tx_handles| tx_handles.remove(to));
    }

    /// - `Some(())` => Ok
    /// - `None` => Port not found
    pub fn toggle_read_state(&self, name: &str) -> Option<()> {
        tracing::debug!(name=%name, "Toggling read state");

        return self.open_serial_ports.read().get(name).map(|port| {
            port.set_read_state(port.read_state().toggle());
        });
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ManagedSerialPortsError {
    #[error("Failed to get available ports: {0}")]
    AvailablePortsError(
        #[source]
        #[from]
        AvailablePortsError,
    ),
}
