use crate::{
    app::state::AppManagedSerialPortsError,
    tauri_app::{model::managed_serial_port::ManagedSerialPort, state::TauriAppState},
};

pub async fn toggle_read_state_intern(
    name: &str,
    state: &TauriAppState,
) -> Result<Vec<ManagedSerialPort>, ToggleReadStateError> {
    tracing::info!(name=%name, "Toggling read state");

    state
        .serial_state()
        .toggle_read_state(name)
        .await
        .ok_or(ToggleReadStateError::NotOpen)?;

    let managed_serial_ports = state.get_managed_serial_ports().await?;

    Ok(managed_serial_ports)
}

#[derive(Debug, thiserror::Error)]
pub enum ToggleReadStateError {
    #[error("Port not open")]
    NotOpen,
    #[error("Failed to get managed ports: {0}")]
    ManagedSerialPortsError(
        #[source]
        #[from]
        AppManagedSerialPortsError,
    ),
}
