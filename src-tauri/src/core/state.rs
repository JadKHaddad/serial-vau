use std::{collections::HashMap, ops::Deref, sync::Arc};

use error::{ManagedSerialPortsError, OpenSerialPortError, PacketError};
use futures::{SinkExt, StreamExt};
#[cfg(feature = "subscriptions")]
use open_serial_port::TxHandle;
use open_serial_port::{
    IncomingPacket, OpenSerialPort, OpenSerialPortOptions, OutgoingPacket, Packet, PacketDirection,
    PacketOrigin, SendError, SubscriptionPacketOrigin,
};
use parking_lot::RwLock;
use tokio::sync::mpsc::UnboundedReceiver as MPSCUnboundedReceiver;
use tokio_serial::{DataBits, FlowControl, Parity, SerialPortBuilderExt, StopBits};
use tokio_util::{
    bytes::BytesMut,
    codec::{BytesCodec, Decoder, FramedRead, FramedWrite, LinesCodec},
    sync::CancellationToken,
};

use super::serial::{managed_serial_port::OpenStatus, SerialPort};

use super::serial::managed_serial_port::{ManagedSerialPort, Status};

pub mod error;
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

/// - `Key`: Serial port name.
/// - `Value`: Open serial port [`OpenSerialPort`].
type OpenSerialPorts = HashMap<String, OpenSerialPort>;

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
#[derive(Debug, Default)]
pub struct AppStateInner {
    /// Not using an async `RwLock` because [`WMIConnection`](wmi::WMIConnection) is not [`Send`],
    /// which is used in [`Watcher`](super::serial::watcher::Watcher),
    /// which is used in [`run`](crate::app::run).
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

impl AppStateInner {
    pub fn managed_serial_ports(&self) -> Result<Vec<ManagedSerialPort>, ManagedSerialPortsError> {
        let available_serial_ports = super::serial::available_ports()?;
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
                };

                if let Some(open_serial_port) = open_serial_ports.get(port.name()) {
                    managed_serial_port.status = Status::Open(OpenStatus {
                        read_state: open_serial_port.read_state(),
                    });
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
    fn subscriptions(&self) -> Arc<RwLock<Subscriptions>> {
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

impl AppState {
    pub async fn open_serial_port(
        &self,
        options: OpenSerialPortOptions,
    ) -> Result<MPSCUnboundedReceiver<Result<Packet, PacketError>>, OpenSerialPortError> {
        tracing::debug!(?options, "Opening serial port");

        let port_to_open_name = self
            .is_port_closed(&options.name)?
            .ok_or(OpenSerialPortError::NotFound)?
            .then_some(&options.name)
            .ok_or(OpenSerialPortError::AlreadyOpen)?;

        let port = tokio_serial::new(port_to_open_name, 115200)
            .stop_bits(StopBits::One)
            .data_bits(DataBits::Eight)
            .flow_control(FlowControl::None)
            .parity(Parity::None)
            .open_native_async()?;

        let (port_read, port_write) = tokio::io::split(port);
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<OutgoingPacket>();

        let (packet_tx, packet_rx) =
            tokio::sync::mpsc::unbounded_channel::<Result<Packet, PacketError>>();

        let cancellation_token = CancellationToken::new();

        let mut framed_read_bytes_port = FramedRead::new(port_read, BytesCodec::new());
        let mut framed_write_bytes_port = FramedWrite::new(port_write, BytesCodec::new());

        let (read_state_tx, mut read_state_rx) =
            tokio::sync::watch::channel(options.initial_read_state);

        self.add_open_serial_port(OpenSerialPort::new(
            SerialPort::new(options.name.clone()),
            tx,
            cancellation_token.clone(),
            read_state_tx,
        ));

        #[cfg(feature = "subscriptions")]
        let subscriptions = self.subscriptions();
        let read_app_state = self.clone();
        let read_cancellation_token = cancellation_token.clone();
        let read_name = options.name.clone();
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
                                                    if let Some(subscriptions) = subscriptions.read().get(&read_name){
                                                        for (subscriber_name, tx_handle) in subscriptions {
                                                            if let Some(tx_handle) = tx_handle {
                                                                tracing::trace!(target: "serial_core::serial::read::byte::subscribe", name=%read_name, subscriber=%subscriber_name, "Sending bytes to subscriber");

                                                                let outgoing_packet = OutgoingPacket {
                                                                    bytes: bytes.clone().into(),
                                                                    packet_origin: PacketOrigin::Subscription(SubscriptionPacketOrigin{ name: read_name.clone() }),
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
                                                                tracing::trace!(target: "serial_core::serial::read::line", name=%read_name, %line, "Read");

                                                                let packet = Packet::new_with_current_timestamp(
                                                                    PacketDirection::Incoming(
                                                                        IncomingPacket {
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
                                                                let _ = read_packet_tx.send(Err(PacketError::Incoming(err.into())));

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
                                                    let _ = read_packet_tx.send(Err(PacketError::Incoming(err.into())));

                                                    // Removing the port will drop the sender causing the write loop to break.
                                                    tracing::debug!(target: "serial_core::serial::read", name=%read_name, "Removing serial port due to an error");
                                                    read_app_state.remove_open_serial_port(&read_name);

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

        let write_name = options.name.clone();
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

                                let packet = Packet::new_with_current_timestamp(
                                    PacketDirection::Outgoing(
                                        OutgoingPacket {
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
                                let _ = write_packet_tx.send(Err(PacketError::Outgoing(err.into())));

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
