use std::{collections::HashMap, ops::Deref, sync::Arc};

use open_serial_port::{OpenSerialPort, SendError, TxHandle};
use parking_lot::RwLock;
use tokio_util::bytes::Bytes;

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
    /// Not using an async `RwLock` because [`WMIConnection`](wmi::WMIConnection) is not [`Send`],
    /// which is used in [`Watcher`](crate::serial::watcher::Watcher),
    /// which is used in [`run`](crate::app::run).
    open_serial_ports: RwLock<HashMap<String, OpenSerialPort>>,
    subscriptions: Arc<RwLock<HashMap<String, HashMap<String, Option<TxHandle>>>>>,
}

impl AppStateInner {
    pub fn managed_serial_ports(&self) -> Result<Vec<ManagedSerialPort>, ManagedSerialPortsError> {
        // TODO: if a subscriber got removed, we will get send errors in open_serial_port
        let available_serial_ports = crate::serial::available_ports()?;
        let open_serial_ports = self.open_serial_ports.read();
        let subscriptions = self.subscriptions.read();

        let managed_serial_ports = available_serial_ports
            .into_iter()
            .map(|port| {
                let subscribed_to = subscriptions
                    .iter()
                    .filter(|&(_, tx_handles)| tx_handles.contains_key(port.name()))
                    .map(|(name, _)| name.clone())
                    .collect::<Vec<_>>();

                let subscriptions = subscriptions
                    .get(port.name())
                    .unwrap_or(&HashMap::new())
                    .iter()
                    .map(|(name, _)| name.clone())
                    .collect();

                let mut managed_serial_port = ManagedSerialPort {
                    name: port.name().to_string(),
                    status: Status::Closed,
                    subscriptions,
                    subscribed_to,
                };

                if open_serial_ports.contains_key(port.name()) {
                    managed_serial_port.status = Status::Open
                }

                managed_serial_port
            })
            .collect::<Vec<_>>();

        Ok(managed_serial_ports)
    }

    pub fn add_open_serial_port(&self, open_serial_port: OpenSerialPort) -> Option<OpenSerialPort> {
        tracing::debug!(name=%open_serial_port.name(), "Adding serial port");

        self.add_open_serial_port_to_pending_subscriptions(&open_serial_port);

        self.open_serial_ports
            .write()
            .insert(open_serial_port.name().to_string(), open_serial_port)
    }

    fn add_open_serial_port_to_pending_subscriptions(&self, open_serial_port: &OpenSerialPort) {
        tracing::debug!(name=%open_serial_port.name(), "Adding serial port to pending subscriptions");

        let mut subscriptions = self.subscriptions.write();

        for (_, tx_handles) in subscriptions.iter_mut() {
            tx_handles
                .get_mut(open_serial_port.name())
                .map(|tx_handle| tx_handle.replace(open_serial_port.tx_handle()));
        }
    }

    fn remove_from_all_subscriptions(&self, name_to_remove: &str) {
        tracing::debug!(name=%name_to_remove, "Removing serial port as subscriber from all subscriptions");

        let mut subscriptions = self.subscriptions.write();

        for (_, tx_handles) in subscriptions.iter_mut() {
            tx_handles.remove(name_to_remove);
        }
    }

    pub fn remove_open_serial_port(&self, name: &str) -> Option<OpenSerialPort> {
        tracing::debug!(name=%name, "Removing serial port");

        self.remove_from_all_subscriptions(name);
        self.open_serial_ports.write().remove(name)
    }

    pub fn remove_and_cancel_open_serial_port(&self, name: &str) -> Option<OpenSerialPort> {
        self.remove_open_serial_port(name)
            .map(OpenSerialPort::cancelled)
    }

    /// Ok(Some(bool)) => Port found
    /// Ok(None) => Port not found
    pub fn is_port_open(&self, name: &str) -> Result<Option<bool>, ManagedSerialPortsError> {
        let managed_serial_ports = self.managed_serial_ports()?;
        let managed_serial_port = managed_serial_ports.iter().find(|port| port.name == name);

        Ok(managed_serial_port.map(|port| port.is_open()))
    }

    /// Ok(Some(bool)) => Port found
    /// Ok(None) => Port not found
    pub fn is_port_closed(&self, name: &str) -> Result<Option<bool>, ManagedSerialPortsError> {
        let managed_serial_ports = self.managed_serial_ports()?;
        let managed_serial_port = managed_serial_ports.iter().find(|port| port.name == name);

        Ok(managed_serial_port.map(|port| port.is_closed()))
    }

    /// Some(Ok()) => Ok
    /// Some(Err(_)) => Send error
    /// None => Port not found
    pub fn send_to_open_serial_port(
        &self,
        name: &str,
        value: Bytes,
    ) -> Option<Result<(), SendError>> {
        Some(self.open_serial_ports.read().get(name)?.send(value))
    }

    pub fn send_to_all_open_serial_ports(&self, value: Bytes) {
        self.open_serial_ports.read().values().for_each(|port| {
            // Cheap clone
            let _ = port.send(value.clone());
        })
    }

    pub fn subscriptions(&self) -> Arc<RwLock<HashMap<String, HashMap<String, Option<TxHandle>>>>> {
        self.subscriptions.clone()
    }

    pub fn subscribe(&self, from: &str, to: &str) {
        tracing::debug!(%from, %to, "Subscribing");

        let mut subscriptions = self.subscriptions.write();

        let tx_handle = self
            .open_serial_ports
            .read()
            .get(from)
            .map(|port| port.tx_handle());

        subscriptions
            .entry(from.to_string())
            .or_default()
            .insert(to.to_string(), tx_handle);
    }

    pub fn unsubscribe(&self, from: &str, to: &str) {
        tracing::debug!(%from, %to, "Unsubscribing");

        let mut subscriptions = self.subscriptions.write();

        subscriptions
            .get_mut(from)
            .and_then(|tx_handles| tx_handles.remove(to));
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
