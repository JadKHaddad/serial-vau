use serde::{Deserialize, Serialize};
use tokio::sync::{
    mpsc::{error::SendError as TokioSendError, UnboundedSender as MPSCUnboundedSender},
    watch::Sender as WatchSender,
};
use tokio_util::{bytes::Bytes, sync::CancellationToken};

use crate::serial::SerialPort;

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
pub struct TxHandle {
    serial_port: SerialPort,
    tx: MPSCUnboundedSender<Bytes>,
}

impl TxHandle {
    pub fn send(&self, value: Bytes) -> Result<(), SendError> {
        Ok(self.tx.send(value)?)
    }

    pub fn name(&self) -> &str {
        self.serial_port.name()
    }
}

#[derive(Debug)]
pub struct OpenSerialPort {
    serial_port: SerialPort,
    tx: MPSCUnboundedSender<Bytes>,
    cancellation_token: CancellationToken,
    read_state_tx: WatchSender<ReadState>,
}

impl OpenSerialPort {
    pub fn new(
        serial_port: SerialPort,
        tx: MPSCUnboundedSender<Bytes>,
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

    pub(super) fn send(&self, value: Bytes) -> Result<(), SendError> {
        Ok(self.tx.send(value)?)
    }

    pub(super) fn tx_handle(&self) -> TxHandle {
        TxHandle {
            serial_port: self.serial_port.clone(),
            tx: self.tx.clone(),
        }
    }

    /// Fails silently if the send fails. Open serial Port is probably closed.
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
        TokioSendError<Bytes>,
    ),
}
