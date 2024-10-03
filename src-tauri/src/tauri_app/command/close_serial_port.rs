use crate::{
    app::state::error::AppManagedSerialPortsError,
    tauri_app::{model::managed_serial_port::ManagedSerialPort, state::TauriAppState},
};

pub async fn close_serial_port_intern(
    name: String,
    state: &TauriAppState,
) -> Result<Vec<ManagedSerialPort>, CloseSerialPortError> {
    tracing::info!(name=%name, "Closing serial port");

    let _ = state
        .serial_state()
        .remove_and_cancel_open_serial_port(&name)
        .await
        .ok_or(CloseSerialPortError::NotOpen)?;

    let managed_serial_ports = state.get_managed_serial_ports().await?;

    Ok(managed_serial_ports)
}

#[derive(Debug, thiserror::Error)]
pub enum CloseSerialPortError {
    #[error("Port not open")]
    NotOpen,
    #[error("Failed to get managed ports: {0}")]
    ManagedSerialPortsError(
        #[source]
        #[from]
        AppManagedSerialPortsError,
    ),
}
