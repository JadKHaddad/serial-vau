use tauri::{AppHandle, Manager};

use crate::tauri_app::{
    event::model::managed_serial_ports::ManagedSerialPortsEvent,
    state::{TauriAppState, TauriAppStateManagedSerialPortsError},
};

pub async fn emit_managed_serial_ports(
    app: &AppHandle,
    state: &TauriAppState,
) -> Result<(), EmitManagedSerialPortsError> {
    tracing::info!("Emitting serial ports");

    let managed_serial_ports = state.get_managed_serial_ports().await?;

    tracing::debug!(?managed_serial_ports);

    let managed_serial_ports_event = ManagedSerialPortsEvent {
        ports: managed_serial_ports.into_iter().map(Into::into).collect(),
    };

    app.emit_all("serial_ports_event", &managed_serial_ports_event)?;

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum EmitManagedSerialPortsError {
    #[error("Failed to get managed ports: {0}")]
    ManagedSerialPortsError(
        #[source]
        #[from]
        TauriAppStateManagedSerialPortsError,
    ),
    #[error("Failed to emit: {0}")]
    EmitError(
        #[source]
        #[from]
        tauri::Error,
    ),
}
