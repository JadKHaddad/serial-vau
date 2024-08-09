use crate::{
    app::model::managed_serial_port::ManagedSerialPort,
    core::state::{error::ManagedSerialPortsError, AppState},
};

pub fn get_serial_ports_intern(
    state: &AppState,
) -> Result<Vec<ManagedSerialPort>, GetSerialPortsError> {
    tracing::info!("Getting serial ports");

    let managed_serial_ports = state
        .managed_serial_ports()?
        .into_iter()
        .map(Into::into)
        .collect();

    tracing::debug!(?managed_serial_ports);

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
