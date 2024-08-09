use crate::{
    app::model::managed_serial_port::ManagedSerialPort,
    core::state::{error::ManagedSerialPortsError, AppState},
};

pub async fn close_serial_port_intern(
    name: String,
    state: &AppState,
) -> Result<Vec<ManagedSerialPort>, CloseSerialPortError> {
    tracing::info!(name=%name, "Closing serial port");

    let _ = state
        .remove_and_cancel_open_serial_port(&name)
        .ok_or(CloseSerialPortError::NotOpen)?;

    let managed_serial_ports = state.managed_serial_ports()?;
    let managed_serial_ports = managed_serial_ports.into_iter().map(Into::into).collect();

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
        ManagedSerialPortsError,
    ),
}
