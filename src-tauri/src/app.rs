use anyhow::Context;
use tauri::{AppHandle, Manager};

use crate::serial::SerialPort;

pub mod state;

#[tauri::command]
pub fn refresh_serial_ports(app: AppHandle) -> Result<(), String> {
    tracing::info!("Refreshing serial ports");

    let ports = crate::serial::available_ports().map_err(|err| err.to_string())?;
    app.emit_all("serial_ports_event", &ports)
        .map_err(|err| err.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn get_serial_ports() -> Result<Vec<SerialPort>, String> {
    tracing::info!("Getting serial ports");

    let ports = crate::serial::available_ports().map_err(|err| err.to_string())?;

    Ok(ports)
}

pub fn run() -> anyhow::Result<()> {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle_creation = app.app_handle().clone();
            let app_handle_deletion = app.app_handle().clone();

            std::thread::spawn(move || {
                tracing::debug!("Starting serial creation events watcher");

                let con = crate::wmi::Con::new()?;
                let creation_iter = con.creation_iter()?;

                for serial_port in creation_iter {
                    let Ok(serial_port) = serial_port else {
                        break;
                    };

                    tracing::trace!(name=%serial_port.name(), "Serial creation event detected");

                    if let Ok(ports) = crate::serial::available_ports() {
                        let _ = app_handle_creation.emit_all("serial_ports_event", &ports);
                    }
                }

                anyhow::Result::<()>::Ok(())
            });

            std::thread::spawn(move || {
                tracing::debug!("Starting serial deletion events watcher");

                let con = crate::wmi::Con::new()?;
                let deletion_iter = con.deletion_iter()?;

                for serial_port in deletion_iter {
                    let Ok(serial_port) = serial_port else {
                        break;
                    };

                    tracing::trace!(name=%serial_port.name(), "Serial deletion event detected");

                    if let Ok(ports) = crate::serial::available_ports() {
                        let _ = app_handle_deletion.emit_all("serial_ports_event", &ports);
                    }
                }

                anyhow::Result::<()>::Ok(())
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_serial_ports,
            refresh_serial_ports
        ])
        .run(tauri::generate_context!())
        .context("Error while running tauri application")
}
