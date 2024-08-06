use tauri::{AppHandle, Manager};

use crate::{
    app::{error::AppError, event::managed_serial_ports::ManagedSerialPortsEvent},
    core::state::AppState,
};

pub fn refresh_serial_ports_intern(app: &AppHandle, state: &AppState) -> Result<(), AppError> {
    tracing::info!("Refreshing serial ports");

    let managed_serial_ports = state.managed_serial_ports()?;

    tracing::debug!(?managed_serial_ports);

    let managed_serial_ports_event = ManagedSerialPortsEvent {
        ports: managed_serial_ports.into_iter().map(Into::into).collect(),
    };

    app.emit_all("serial_ports_event", &managed_serial_ports_event)?;

    Ok(())
}
