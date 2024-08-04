use serde::{Deserialize, Serialize};
use tokio::sync::{
    mpsc::{error::SendError as TokioSendError, UnboundedSender as MPSCUnboundedSender},
    watch::Sender as WatchSender,
};
use tokio_util::{bytes::Bytes, sync::CancellationToken};

use crate::serial::SerialPort;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PacketOrigin {
    /// Sent directly to the serial port by he user.
    Direct,
    /// Sent via a broadcast to all open serial ports.
    Broadcast,
    /// Sent via a subscription from another serial port.
    Subscription { from: String },
}

/// Usefull for tracing.
impl std::fmt::Display for PacketOrigin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Direct => write!(f, "Direct"),
            Self::Broadcast => write!(f, "Broadcast"),
            Self::Subscription { from } => write!(f, "Subscription from: [{}]", from),
        }
    }
}

/// Represents a packet that is sent to a serial port.
#[derive(Debug, Clone)]
pub struct OutgoingPacket {
    pub data: Bytes,
    pub origin: PacketOrigin,
    timestamp_millis: u64,
}

impl OutgoingPacket {
    pub fn new_with_current_timestamp(data: Bytes, origin: PacketOrigin) -> Self {
        Self {
            data,
            origin,
            timestamp_millis: chrono::Utc::now().timestamp_millis() as u64,
        }
    }

    pub fn timestamp_millis(&self) -> u64 {
        self.timestamp_millis
    }
}

/// Defines if an open serial port is currently reading or stopped.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ReadState {
    Read,
    Stop,
}

impl ReadState {
    pub fn is_stop(&self) -> bool {
        matches!(self, Self::Stop)
    }

    pub fn toggle(self) -> Self {
        match self {
            Self::Read => Self::Stop,
            Self::Stop => Self::Read,
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

#[derive(Debug, thiserror::Error)]
pub enum SendError {
    #[error("Failed to send: {0}")]
    Send(
        #[source]
        #[from]
        TokioSendError<OutgoingPacket>,
    ),
}
