use tauri::{AppHandle, Manager};

use crate::{
    app::state::error::AppManagedSerialPortsError,
    tauri_app::{
        event::{events::SERIAL_PORTS_EVENT, model::managed_serial_ports::ManagedSerialPortsEvent},
        state::TauriAppState,
    },
};

pub async fn emit_managed_serial_ports_event(
    app: &AppHandle,
    state: &TauriAppState,
) -> Result<(), EmitManagedSerialPortsError> {
    tracing::debug!("Emitting serial ports");

    let managed_serial_ports = state.get_managed_serial_ports().await?;

    let managed_serial_ports_event = ManagedSerialPortsEvent {
        ports: managed_serial_ports,
    };

    app.emit_all(SERIAL_PORTS_EVENT, &managed_serial_ports_event)?;

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
