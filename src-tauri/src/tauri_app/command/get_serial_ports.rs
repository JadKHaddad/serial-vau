use crate::{
    core::state::{error::ManagedSerialPortsError, State},
    tauri_app::model::managed_serial_port::ManagedSerialPort,
};

pub fn get_serial_ports_intern(
    state: &State,
) -> Result<Vec<ManagedSerialPort>, GetSerialPortsError> {
    tracing::info!("Getting serial ports");

    let managed_serial_ports = state.managed_serial_ports()?;

    tracing::debug!(?managed_serial_ports);

    let managed_serial_ports = managed_serial_ports.into_iter().map(Into::into).collect();

    Ok(managed_serial_ports)
}

#[derive(Debug, thiserror::Error)]
pub enum GetSerialPortsError {
    #[error("Failed to get managed ports: {0}")]
    ManagedSerialPortsError(
        #[source]
        #[from]
        ManagedSerialPortsError,
    ),
}
