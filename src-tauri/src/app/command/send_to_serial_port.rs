use tokio_util::bytes::Bytes;

use crate::app::state::{open_serial_port::SendError, AppState};

pub fn send_to_serial_port_intern(
    name: String,
    value: Bytes,
    state: &AppState,
) -> Result<(), SendToSerialPortError> {
    tracing::info!(name=%name, "Sending to serial port");

    Ok(state
        .send_to_open_serial_port(&name, value)
        .ok_or(SendToSerialPortError::NotOpen)??)
}

#[derive(Debug, thiserror::Error)]
pub enum SendToSerialPortError {
    #[error("Port not open")]
    NotOpen,
    #[error("Failed to send: {0}")]
    SendError(
        #[source]
        #[from]
        SendError,
    ),
}
