use tokio::sync::mpsc::{error::SendError as TokioSendError, UnboundedSender};
use tokio_util::{bytes::Bytes, sync::CancellationToken};

use crate::serial::SerialPort;

#[derive(Debug)]
pub struct OpenSerialPort {
    serial_port: SerialPort,
    tx: UnboundedSender<Bytes>,
    cancellation_token: CancellationToken,
}

#[derive(Debug)]
pub struct TxHandle {
    serial_port: SerialPort,
    tx: UnboundedSender<Bytes>,
}

impl TxHandle {
    pub fn send(&self, value: Bytes) -> Result<(), SendError> {
        Ok(self.tx.send(value)?)
    }

    pub fn name(&self) -> &str {
        self.serial_port.name()
    }
}

impl OpenSerialPort {
    pub fn new(
        serial_port: SerialPort,
        tx: UnboundedSender<Bytes>,
        cancellation_token: CancellationToken,
    ) -> Self {
        Self {
            serial_port,
            tx,
            cancellation_token,
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
