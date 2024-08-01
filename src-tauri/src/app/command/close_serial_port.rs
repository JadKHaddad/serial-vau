use serde::Deserialize;

use crate::app::state::AppState;

#[derive(Debug, Deserialize)]
pub struct OpenSerialPortOptions {
    name: String,
}

pub async fn close_serial_port_intern(
    name: String,
    state: &AppState,
) -> Result<(), CloseSerialPortError> {
    tracing::info!(name=%name, "Closing serial port");

    let _ = state
        .remove_and_cancel_open_serial_port(&name)
        .ok_or(CloseSerialPortError::NotOpen)?;

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum CloseSerialPortError {
    #[error("Port not open")]
    NotOpen,
}
