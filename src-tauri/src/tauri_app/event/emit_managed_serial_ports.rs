use tauri::{AppHandle, Manager};

use crate::{
    app::state::error::AppManagedSerialPortsError,
    tauri_app::{
        event::model::managed_serial_ports::ManagedSerialPortsEvent, state::TauriAppState,
    },
};

pub async fn emit_managed_serial_ports(
    app: &AppHandle,
    state: &TauriAppState,
) -> Result<(), EmitManagedSerialPortsError> {
    tracing::info!("Emitting serial ports");

    let managed_serial_ports = state.get_managed_serial_ports().await?;

    let managed_serial_ports_event = ManagedSerialPortsEvent {
        ports: managed_serial_ports,
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
        AppManagedSerialPortsError,
    ),
    #[error("Failed to emit: {0}")]
    EmitError(
        #[source]
        #[from]
        tauri::Error,
    ),
}
