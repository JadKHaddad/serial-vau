use tokio::sync::{
    mpsc::{error::SendError as TokioSendError, UnboundedSender as MPSCUnboundedSender},
    watch::Sender as WatchSender,
};
use tokio_util::{bytes::Bytes, sync::CancellationToken};

use crate::core::serial::managed_serial_port::ReadState;

use super::SerialPort;

/// Describes how a given serial port should be open.
#[derive(Debug)]
pub struct OpenSerialPortOptions {
    /// Name of the serial port.
    pub name: String,
    /// Defines the [`ReadState`] of a serial port before it is even open.
    pub initial_read_state: ReadState,
    // TODO: Other fields: BaudRate ...
}

/// Represents a packet that is received from a serial port.
#[derive(Debug, Clone)]
pub struct IncomingPacket {
    pub line: Bytes,
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone)]
pub struct SubscriptionPacketOrigin {
    /// The name of the serial port that sent the packet.
    pub name: String,
}

/// Origin of an [`OutgoingPacket`].
#[derive(Debug, Clone)]
pub enum PacketOrigin {
    /// Sent directly to the serial port by he user.
    Direct,
    /// Sent via a broadcast to all open serial ports.
    Broadcast,
    /// Sent via a subscription from another serial port.
    #[cfg(feature = "subscriptions")]
    Subscription(SubscriptionPacketOrigin),
}

/// Usefull for tracing.
impl std::fmt::Display for PacketOrigin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Direct => write!(f, "Direct"),
            Self::Broadcast => write!(f, "Broadcast"),
            #[cfg(feature = "subscriptions")]
            Self::Subscription(subscription) => {
                write!(f, "Subscription from: [{}]", subscription.name)
            }
        }
    }
}

/// Represents a packet that is sent to a serial port.
#[derive(Debug, Clone)]
pub struct OutgoingPacket {
    /// Bytes sent.
    pub bytes: Bytes,
    /// Origin of an [`OutgoingPacket`].
    pub packet_origin: PacketOrigin,
}

#[derive(Debug, Clone)]
pub enum PacketDirection {
    /// From the open serial port to the application.
    Incoming(IncomingPacket),
    /// From the application to the open serial port.
    Outgoing(OutgoingPacket),
}

/// Packet emitted by [`AppState::open_serial_port`](crate::core::state::AppState::open_serial_port) through the channel.
#[derive(Debug, Clone)]
pub struct Packet {
    pub packet_direction: PacketDirection,
    /// The name of the corresponding serial port.
    pub port_name: String,
    pub timestamp_millis: u64,
}

impl Packet {
    pub fn new_with_current_timestamp(
        packet_direction: PacketDirection,
        port_name: String,
    ) -> Self {
        Self {
            packet_direction,
            port_name,
            timestamp_millis: chrono::Utc::now().timestamp_millis() as u64,
        }
    }
}

/// Used to copy the [`OpenSerialPort::tx`] field from [`OpenSerialPort`].
/// Used as a handle to send data to a serial port that is a subscriber to another serial port.
#[derive(Debug)]
#[cfg(feature = "subscriptions")]
pub struct TxHandle {
    serial_port: SerialPort,
    tx: MPSCUnboundedSender<OutgoingPacket>,
}

#[cfg(feature = "subscriptions")]
impl TxHandle {
    pub fn send(&self, value: OutgoingPacket) -> Result<(), SendError> {
        Ok(self.tx.send(value)?)
    }

    pub fn name(&self) -> &str {
        self.serial_port.name()
    }
}

#[derive(Debug)]
pub struct OpenSerialPort {
    serial_port: SerialPort,
    /// Main channel to send data to the serial port.
    ///
    /// The write task is waiting for data to be sent to the serial port.
    tx: MPSCUnboundedSender<OutgoingPacket>,
    cancellation_token: CancellationToken,
    /// Defines if the read task is currently reading or stopped.
    ///
    /// The read task is always watching for changes to the read state.
    read_state_tx: WatchSender<ReadState>,
}

impl OpenSerialPort {
    pub fn new(
        serial_port: SerialPort,
        tx: MPSCUnboundedSender<OutgoingPacket>,
        cancellation_token: CancellationToken,
        read_state_tx: WatchSender<ReadState>,
    ) -> Self {
        Self {
            serial_port,
            tx,
            cancellation_token,
            read_state_tx,
        }
    }

    pub fn name(&self) -> &str {
        self.serial_port.name()
    }

    fn cancel(&self) {
        tracing::debug!(name=%self.name(), "Cancelling");

        self.cancellation_token.cancel()
    }

    pub(super) fn cancelled(self) -> Self {
        self.cancel();
        self
    }

    pub(super) fn send(&self, value: OutgoingPacket) -> Result<(), SendError> {
        Ok(self.tx.send(value)?)
    }

    #[cfg(feature = "subscriptions")]
    pub(super) fn tx_handle(&self) -> TxHandle {
        TxHandle {
            serial_port: self.serial_port.clone(),
            tx: self.tx.clone(),
        }
    }

    /// Fails silently if the send fails. Open serial port is probably closed.
    pub(super) fn set_read_state(&self, read_state: ReadState) {
        let _ = self.read_state_tx.send(read_state);
    }

    pub(super) fn read_state(&self) -> ReadState {
        *self.read_state_tx.borrow()
    }
}

/// Error returned by [`OpenSerialPort::send`](OpenSerialPort::send) and [`TxHandle::send`](TxHandle::send)
#[derive(Debug, thiserror::Error)]
pub enum SendError {
    #[error("Failed to send: {0}")]
    Send(
        #[source]
        #[from]
        TokioSendError<OutgoingPacket>,
    ),
}
