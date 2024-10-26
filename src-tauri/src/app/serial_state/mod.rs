use std::{collections::HashMap, ops::Deref, sync::Arc};

use error::{CoreManagedSerialPortsError, CoreOpenSerialPortError, CorePacketError};
use futures::{SinkExt, StreamExt};
use handle::{CoreOpenSerialPort, SendError, TxHandle};
use model::{
    CoreManagedSerialPort, CoreOpenStatus, CoreSerialPort, Status,
    {
        CoreIncomingPacket, CoreOpenSerialPortOptions, CoreOutgoingPacket, CorePacket,
        CorePacketDirection,
    },
};
#[cfg(feature = "subscriptions")]
use model::{CorePacketOrigin, CoreSubscriptionPacketOrigin};
use tokio::sync::{mpsc::UnboundedReceiver as MPSCUnboundedReceiver, RwLock};
use tokio_util::{
    bytes::BytesMut,
    codec::{BytesCodec, Decoder, FramedRead, FramedWrite},
    sync::CancellationToken,
};

use crate::serial_manager::{serial_manager_service::SerialManagerService, SerialManager};

use codec::lines_codec::LinesCodec;

pub mod codec;
pub mod error;
pub mod handle;
pub mod model;

#[derive(Debug, Clone)]
pub struct CoreSerialState {
    inner: Arc<StateInner>,
}

impl CoreSerialState {
    pub fn new(manager: SerialManager) -> Self {
        Self {
            inner: Arc::new(StateInner::new(manager)),
        }
    }
}

impl Deref for CoreSerialState {
    type Target = StateInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// - `Key`: Serial port name.
/// - `Value`: Open serial port [`CoreOpenSerialPort`].
type OpenSerialPorts = HashMap<String, CoreOpenSerialPort>;

/// - `Key`: Master Serial port name.
/// - `Value`:  
///     - `Key`: Subscriber serial port name.
///     - `Value`: Optional subscriber's [`TxHandle`] to send data to the subscriber.
///        - `Some(TxHandle)`: Subscriber is open.
///        - `None`: Subscriber is closed.
#[cfg(feature = "subscriptions")]
type Subscriptions = HashMap<String, HashMap<String, Option<TxHandle>>>;

/// ## Note
/// Locks are not optimized. See branch [`feat/optimize-locks`](https://github.com/JadKHaddad/serial-vau/tree/feat/optimize-locks) for optimized locks sacrificing readability.
#[derive(Debug)]
pub struct StateInner {
    manager: SerialManager,
    open_serial_ports: RwLock<OpenSerialPorts>,
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
    subscriptions: Arc<RwLock<Subscriptions>>,
}

impl StateInner {
    fn new(manager: SerialManager) -> Self {
        Self {
            manager,
            open_serial_ports: RwLock::new(HashMap::new()),
            #[cfg(feature = "subscriptions")]
            subscriptions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// ## Locks
    ///
    /// - Read: [`Self::open_serial_ports`]
    #[cfg_attr(
        feature = "subscriptions",
        doc = "
- Read: [`Self::subscriptions`].
    "
    )]
    pub async fn managed_serial_ports(
        &self,
    ) -> Result<Vec<CoreManagedSerialPort>, CoreManagedSerialPortsError> {
        let available_serial_ports = self.manager.available_ports()?;
        let open_serial_ports = self.open_serial_ports.read().await;
        #[cfg(feature = "subscriptions")]
        let subscriptions = self.subscriptions.read().await;

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

                let mut managed_serial_port = CoreManagedSerialPort {
                    name: port.name().to_string(),
                    status: Status::Closed,
                    #[cfg(feature = "subscriptions")]
                    subscriptions,
                    #[cfg(feature = "subscriptions")]
                    subscribed_to,
                };

                if let Some(open_serial_port) = open_serial_ports.get(port.name()) {
                    managed_serial_port.status = Status::Open(CoreOpenStatus {
                        read_state: open_serial_port.read_state(),
                    });
                }

                managed_serial_port
            })
            .collect::<Vec<_>>();

        Ok(managed_serial_ports)
    }

    /// Adds the serial port to [`Self::open_serial_ports`] and adds it to all subscriptions.
    ///
    /// ## Locks
    ///
    /// - Write: [`Self::open_serial_ports`].
    #[cfg_attr(
        feature = "subscriptions",
        doc = "
- Write: [`Self::subscriptions`]. Inherited from [`Self::add_open_serial_port_to_pending_subscriptions`].
    "
    )]
    async fn add_open_serial_port(
        &self,
        open_serial_port: CoreOpenSerialPort,
    ) -> Option<CoreOpenSerialPort> {
        tracing::debug!(name=%open_serial_port.name(), "Adding serial port");

        #[cfg(feature = "subscriptions")]
        self.add_open_serial_port_to_pending_subscriptions(&open_serial_port)
            .await;

        self.open_serial_ports
            .write()
            .await
            .insert(open_serial_port.name().to_string(), open_serial_port)
    }

    /// Sets the [`Option<TxHandle>`] of the given serial port to `Some(TxHandle)` in all subscriptions.
    ///
    /// A subscription can exist before the subscriber is open.
    ///
    /// ## Locks
    ///
    /// - Write: [`Self::subscriptions`].
    #[cfg(feature = "subscriptions")]
    async fn add_open_serial_port_to_pending_subscriptions(
        &self,
        open_serial_port: &CoreOpenSerialPort,
    ) {
        tracing::debug!(name=%open_serial_port.name(), "Adding serial port to pending subscriptions");

        let mut subscriptions = self.subscriptions.write().await;

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
    ///
    /// ## Locks
    ///
    /// - Write: [`Self::subscriptions`].
    #[cfg(feature = "subscriptions")]
    async fn remove_open_serial_port_from_all_subscriptions(&self, name: &str) {
        tracing::debug!(name=%name, "Removing serial port as subscriber from all subscriptions");

        let mut subscriptions = self.subscriptions.write().await;

        for (_, tx_handles) in subscriptions.iter_mut() {
            tx_handles.get_mut(name).map(|tx_handle| tx_handle.take());
        }
    }

    /// Removes the serial port from [`Self::open_serial_ports`] and cancels its subscription.
    ///
    /// ## Locks
    ///
    #[cfg_attr(
        feature = "subscriptions",
        doc = "
    - Write: [`Self::subscriptions`]. Inherited from [`Self::remove_open_serial_port_from_all_subscriptions`].
    "
    )]
    /// - Write: [`Self::open_serial_ports`].
    async fn remove_open_serial_port(&self, name: &str) -> Option<CoreOpenSerialPort> {
        tracing::debug!(name=%name, "Removing serial port");

        #[cfg(feature = "subscriptions")]
        self.remove_open_serial_port_from_all_subscriptions(name)
            .await;
        self.open_serial_ports.write().await.remove(name)
    }

    /// Removes and cancels the serial port from [`Self::open_serial_ports`] and cancels its subscription.
    ///
    /// ## Locks
    ///
    #[cfg_attr(
        feature = "subscriptions",
        doc = "
    - Write: [`Self::subscriptions`]. Inherited from [`Self::remove_open_serial_port`].
    "
    )]
    /// - Write: [`Self::open_serial_ports`]. Inherited from [`Self::remove_open_serial_port`].
    pub async fn remove_and_cancel_open_serial_port(
        &self,
        name: &str,
    ) -> Option<CoreOpenSerialPort> {
        self.remove_open_serial_port(name)
            .await
            .map(CoreOpenSerialPort::cancelled)
    }

    /// - `Ok(Some(bool))` => Port found.
    /// - `Ok(None)` => Port not found.
    ///
    /// ## Locks
    ///
    /// - Read: [`Self::open_serial_ports`]
    #[cfg_attr(
        feature = "subscriptions",
        doc = "
- Read: [`Self::subscriptions`].
    "
    )]
    async fn is_port_closed(
        &self,
        name: &str,
    ) -> Result<Option<bool>, CoreManagedSerialPortsError> {
        let managed_serial_ports = self.managed_serial_ports().await?;
        let managed_serial_port = managed_serial_ports.iter().find(|port| port.name == name);

        Ok(managed_serial_port.map(|port| port.is_closed()))
    }

    /// - `Some(Ok())` => Ok.
    /// - `Some(Err(_))` => Send error.
    /// - `None` => Port not found.
    ///
    /// ## Locks
    ///
    /// - Read: [`Self::open_serial_ports`].
    pub async fn send_to_open_serial_port(
        &self,
        name: &str,
        packet: CoreOutgoingPacket,
    ) -> Option<Result<(), SendError>> {
        Some(self.open_serial_ports.read().await.get(name)?.send(packet))
    }

    /// ## Locks
    ///
    /// - Read: [`Self::open_serial_ports`].
    pub async fn send_to_all_open_serial_ports(&self, packet: CoreOutgoingPacket) {
        self.open_serial_ports
            .read()
            .await
            .values()
            .for_each(|port| {
                // Cheap clone
                let _ = port.send(packet.clone());
            })
    }

    #[cfg(feature = "subscriptions")]
    fn subscriptions(&self) -> Arc<RwLock<Subscriptions>> {
        self.subscriptions.clone()
    }

    /// `to` is subscribed to `from`.
    ///
    /// - `from` will send data to `to`.
    /// - `to` will receive data from `from`.
    ///
    /// ## Locks
    ///
    /// - Write: [`Self::subscriptions`].
    /// - Read: [`Self::open_serial_ports`].
    #[cfg(feature = "subscriptions")]
    pub async fn subscribe(&self, from: &str, to: &str) {
        tracing::debug!(%from, %to, "Subscribing");

        let mut subscriptions = self.subscriptions.write().await;

        let tx_handle = self
            .open_serial_ports
            .read()
            .await
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
    /// - `to` will no longer receive data from `from`.
    ///
    /// ## Locks
    ///
    /// - Write: [`Self::subscriptions`].
    #[cfg(feature = "subscriptions")]
    pub async fn unsubscribe(&self, from: &str, to: &str) {
        tracing::debug!(%from, %to, "Unsubscribing");

        let mut subscriptions = self.subscriptions.write().await;

        subscriptions
            .get_mut(from)
            .and_then(|tx_handles| tx_handles.remove(to));
    }

    /// - `Some(())` => Ok.
    /// - `None` => Port not found.
    ///
    /// ## Locks
    ///
    /// - Read: [`Self::open_serial_ports`].
    pub async fn toggle_read_state(&self, name: &str) -> Option<()> {
        tracing::debug!(name=%name, "Toggling read state");

        return self.open_serial_ports.read().await.get(name).map(|port| {
            port.set_read_state(port.read_state().toggle());
        });
    }
}

impl CoreSerialState {
    /// ## Locks
    ///
    /// - Write: [`StateInner::open_serial_ports`]. Inherited from [`StateInner::add_open_serial_port`].
    #[cfg_attr(
        feature = "subscriptions",
        doc = "
- Write: [`StateInner::subscriptions`]. Inherited from [`StateInner::add_open_serial_port`].
    "
    )]
    pub async fn open_serial_port<'a>(
        &'a self,
        name: &'a str,
        options: CoreOpenSerialPortOptions,
    ) -> Result<MPSCUnboundedReceiver<Result<CorePacket, CorePacketError>>, CoreOpenSerialPortError>
    {
        tracing::debug!(?options, "Opening serial port");

        let port_to_open_name = self
            .is_port_closed(name)
            .await?
            .ok_or(CoreOpenSerialPortError::NotFound)?
            .then_some(name)
            .ok_or(CoreOpenSerialPortError::AlreadyOpen)?;

        let (initial_read_state, options) = options.split_into_read_state_and_manager_options();

        let port = self.manager.open_port(port_to_open_name, options)?;

        let (port_read, port_write) = tokio::io::split(port);
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<CoreOutgoingPacket>();

        let (packet_tx, packet_rx) =
            tokio::sync::mpsc::unbounded_channel::<Result<CorePacket, CorePacketError>>();

        let cancellation_token = CancellationToken::new();

        let mut framed_read_bytes_port = FramedRead::new(port_read, BytesCodec::new());
        let mut framed_write_bytes_port = FramedWrite::new(port_write, BytesCodec::new());

        let (read_state_tx, mut read_state_rx) = tokio::sync::watch::channel(initial_read_state);

        self.add_open_serial_port(CoreOpenSerialPort::new(
            CoreSerialPort::new(name.into()),
            tx,
            cancellation_token.clone(),
            read_state_tx,
        ))
        .await;

        #[cfg(feature = "subscriptions")]
        let subscriptions = self.subscriptions();
        let read_app_state = self.clone();
        let read_cancellation_token = cancellation_token.clone();
        let read_name = name.to_owned();
        let read_packet_tx = packet_tx.clone();

        tokio::spawn(async move {
            let mut lines_codec = LinesCodec::new();
            let mut lines_bytes = BytesMut::new();

            // Trigger the initial read state.
            read_state_rx.mark_changed();

            loop {
                tracing::debug!(target: "serial_core::serial::read:.watch", name=%read_name, "Waiting for read state change");
                tokio::select! {
                    changed_result = read_state_rx.changed() => {
                        match changed_result {
                            Ok(_) => {
                                let read_state = *read_state_rx.borrow();
                                tracing::debug!(target: "serial_core::serial::read::watch", name=%read_name, ?read_state, "Read state changed");

                                if read_state.is_stop() {

                                    continue;
                                }

                                tracing::debug!(target: "serial_core::serial::read", name=%read_name, "Started reading");
                                loop {
                                    tokio::select! {
                                        bytes = framed_read_bytes_port.next() => {
                                            match bytes {
                                                Some(Ok(bytes)) => {
                                                    tracing::trace!(target: "serial_core::serial::read::byte", name=%read_name, ?bytes, "Read");

                                                    #[cfg(feature = "subscriptions")]
                                                    if let Some(subscriptions) = subscriptions.read().await.get(&read_name){
                                                        for (subscriber_name, tx_handle) in subscriptions {
                                                            if let Some(tx_handle) = tx_handle {
                                                                tracing::trace!(target: "serial_core::serial::read::byte::subscribe", name=%read_name, subscriber=%subscriber_name, "Sending bytes to subscriber");

                                                                let outgoing_packet = CoreOutgoingPacket {
                                                                    bytes: bytes.clone().into(),
                                                                    packet_origin: CorePacketOrigin::Subscription(CoreSubscriptionPacketOrigin{ name: read_name.clone() }),
                                                                };

                                                                if let Err(err) = tx_handle.send(outgoing_packet) {
                                                                    tracing::error!(target: "serial_core::serial::read::byte::subscribe", name=%read_name, subscriber=%subscriber_name, %err, "Failed to send bytes to subscriber");
                                                                }
                                                            }
                                                        }
                                                    }

                                                    lines_bytes.extend_from_slice(&bytes);

                                                    loop {
                                                        match lines_codec.decode(&mut lines_bytes) {
                                                            Ok(None) => break,
                                                            Ok(Some(line)) => {
                                                                tracing::trace!(target: "serial_core::serial::read::line", name=%read_name, ?line, "Read");

                                                                let packet = CorePacket::new_with_current_timestamp(
                                                                    CorePacketDirection::Incoming(
                                                                        CoreIncomingPacket {
                                                                            line,
                                                                        }
                                                                    ),
                                                                    read_name.clone(),
                                                                );

                                                                // Feedback
                                                                let _ = read_packet_tx.send(
                                                                    Ok(packet)
                                                                );

                                                            }
                                                            Err(err) => {
                                                                tracing::warn!(target: "serial_core::serial::read::line", name=%read_name, %err, "Failed to decode line");

                                                                // Feedback
                                                                let _ = read_packet_tx.send(Err(CorePacketError::Incoming(err.into())));

                                                                // Clear the buffer to prevent further errors.
                                                                lines_bytes.clear();

                                                                break;
                                                            }
                                                        }
                                                    }
                                                }
                                                Some(Err(err)) => {
                                                    tracing::error!(target: "serial_core::serial::read", name=%read_name, %err);

                                                    // Feedback
                                                    let _ = read_packet_tx.send(Err(CorePacketError::Incoming(err.into())));

                                                    // Removing the port will drop the sender causing the write loop to break.
                                                    tracing::debug!(target: "serial_core::serial::read", name=%read_name, "Removing serial port due to an error");
                                                    read_app_state.remove_open_serial_port(&read_name).await;

                                                    break;
                                                }
                                                _ => {}
                                            }
                                        },
                                        _ = read_cancellation_token.cancelled() => {
                                            // At this point we should have been removed and cancelled. Nothing to do here.
                                            tracing::debug!(target: "serial_core::serial::read", name=%read_name, "Cancelled");

                                            break;
                                        }
                                        _ = read_state_rx.changed() => {
                                            let read_state = *read_state_rx.borrow();
                                            tracing::debug!(target: "serial_core::serial::read::watch", name=%read_name, ?read_state, "Read state changed");

                                            if read_state.is_stop() {
                                                tracing::debug!(target: "serial_core::serial::read", name=%read_name, "Stopped reading");

                                                break;
                                            }
                                        }
                                    }
                                }
                            }

                            Err(_) => {
                                // Open port was probably removed.
                                tracing::debug!(target: "serial_core::serial::read::watch", name=%read_name, "Read state watch task terminated");

                                break;
                            }
                        }
                    },
                    _ = read_cancellation_token.cancelled() => {
                        tracing::debug!(target: "serial_core::serial::read::watch", name=%read_name, "Cancelled");

                        break;
                    }
                }
            }

            tracing::debug!(target: "serial_core::serial::read", name=%read_name, "Read task terminated")
        });

        let write_name = name.to_owned();
        let write_cancellation_token = cancellation_token;
        let write_packet_tx = packet_tx.clone();

        tokio::spawn(async move {
            // Dropping the sender will automatically break the loop.
            while let Some(packet) = rx.recv().await {
                tracing::trace!(target: "serial_core::serial::write::byte", name=%write_name, origin=%packet.packet_origin, bytes=?packet.bytes, "Sending");
                tracing::trace!(target: "serial_core::serial::write::string", name=%write_name, origin=%packet.packet_origin, bytes=%String::from_utf8_lossy(&packet.bytes), "Sending");

                tokio::select! {
                    // Note: Might get stuck here, therefor the cancellation token.
                    send_result = framed_write_bytes_port.send(packet.bytes.clone()) => {
                        match send_result {
                            Ok(_) => {
                                tracing::trace!(target: "serial_core::serial::write::result", name=%write_name, origin=%packet.packet_origin, "Ok");

                                let packet = CorePacket::new_with_current_timestamp(
                                    CorePacketDirection::Outgoing(
                                        CoreOutgoingPacket {
                                            packet_origin: packet.packet_origin,
                                            bytes: packet.bytes,
                                        }
                                    ),
                                    write_name.clone(),
                                );

                                // Feedback
                                let _ = write_packet_tx.send(Ok(packet));
                            }
                            Err(err) => {
                                // If the write fails we just break out of the loop.
                                // Read task must have also been terminated due to the same error.
                                tracing::error!(target: "serial_core::serial::write::result", name=%write_name, origin=?packet.packet_origin, %err);

                                // Feedback
                                let _ = write_packet_tx.send(Err(CorePacketError::Outgoing(err.into())));

                                break;
                            }
                        }
                    },
                    _ = write_cancellation_token.cancelled() => {
                        tracing::debug!(target: "serial_core::serial::write::result", name=%write_name, "Cancelled");

                        break;
                    }
                }
            }

            tracing::debug!(target: "serial_core::serial::write", name=%write_name, "Write task terminated")
        });

        Ok(packet_rx)
    }
}
