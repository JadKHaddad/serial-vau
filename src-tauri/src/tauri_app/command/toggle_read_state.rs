use crate::{
    core::state::{error::ManagedSerialPortsError, State},
    tauri_app::model::managed_serial_port::ManagedSerialPort,
};

pub fn toggle_read_state_intern(
    name: &str,
    state: &State,
) -> Result<Vec<ManagedSerialPort>, ToggleReadStateError> {
    tracing::info!(name=%name, "Toggling read state");

    state
        .toggle_read_state(name)
        .ok_or(ToggleReadStateError::NotOpen)?;

    let managed_serial_ports = state.managed_serial_ports()?;
    let managed_serial_ports = managed_serial_ports.into_iter().map(Into::into).collect();

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
        ManagedSerialPortsError,
    ),
}
