use tokio::sync::mpsc::{error::SendError, Sender};
use tokio_util::sync::CancellationToken;

use crate::serial::SerialPort;

#[derive(Debug)]
pub struct OpenSerialPort {
    serial_port: SerialPort,
    tx: Sender<String>,
    cancellation_token: CancellationToken,
}

impl OpenSerialPort {
    pub fn new(
        serial_port: SerialPort,
        tx: Sender<String>,
        cancellation_token: CancellationToken,
    ) -> Self {
        Self {
            serial_port,
            tx,
            cancellation_token,
        }
    }

    pub fn name(&self) -> &str {
        &self.serial_port.name()
    }

    pub fn cancel(&self) {
        tracing::debug!(name=%self.name(), "Cancelling");

        self.cancellation_token.cancel()
    }

    pub fn cancelled(self) -> Self {
        self.cancel();
        self
    }

    pub async fn send(&self, value: String) -> Result<(), SendError<String>> {
        self.tx.send(value).await
    }
}
