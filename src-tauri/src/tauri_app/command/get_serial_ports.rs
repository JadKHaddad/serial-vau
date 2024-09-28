use crate::tauri_app::{
    model::managed_serial_port::ManagedSerialPort,
    state::{TauriAppState, TauriAppStateManagedSerialPortsError},
};

pub async fn get_serial_ports_intern(
    state: &TauriAppState,
) -> Result<Vec<ManagedSerialPort>, GetSerialPortsError> {
    tracing::info!("Getting serial ports");

    let managed_serial_ports = state.get_managed_serial_ports().await?;

    tracing::debug!(?managed_serial_ports);

    Ok(managed_serial_ports)
}

#[derive(Debug, thiserror::Error)]
pub enum GetSerialPortsError {
    #[error("Failed to get managed ports: {0}")]
    ManagedSerialPortsError(
        #[source]
        #[from]
        TauriAppStateManagedSerialPortsError,
    ),
}
