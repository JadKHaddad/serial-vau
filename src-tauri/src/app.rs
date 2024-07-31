use anyhow::Context;
use tauri::{AppHandle, Manager};

pub mod state;

#[tauri::command]
pub fn refresh_serial_ports(app: AppHandle) -> Result<(), String> {
    tracing::info!("Refreshing serial ports");

    let ports = crate::serial::available_ports().map_err(|err| err.to_string())?;
    app.emit_all("serial_ports_event", &ports)
        .map_err(|err| err.to_string())?;

    Ok(())
}

pub fn run() -> anyhow::Result<()> {
    tauri::Builder::default()
        .setup(|app| {
            let _ = crate::wmi::spawn_serial_events_watchers(app.app_handle().clone());

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![refresh_serial_ports])
        .run(tauri::generate_context!())
        .context("Error while running tauri application")
}
