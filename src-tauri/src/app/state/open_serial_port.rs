use tokio::sync::mpsc::{error::SendError as TokioSendError, UnboundedSender};
use tokio_util::sync::CancellationToken;

use crate::serial::SerialPort;

#[derive(Debug)]
pub struct OpenSerialPort {
    serial_port: SerialPort,
    tx: UnboundedSender<Vec<u8>>,
    cancellation_token: CancellationToken,
}

impl OpenSerialPort {
    pub fn new(
        serial_port: SerialPort,
        tx: UnboundedSender<Vec<u8>>,
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

    pub(super) fn send(&self, value: Vec<u8>) -> Result<(), SendError> {
        Ok(self.tx.send(value)?)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SendError {
    #[error("Failed to send: {0}")]
    Send(
        #[source]
        #[from]
        TokioSendError<Vec<u8>>,
    ),
}
