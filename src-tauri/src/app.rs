use anyhow::Context;
use error::AppError;
use tauri::{AppHandle, Manager};

use crate::serial::watcher::Watcher as SerialWatcher;

pub mod error;
pub mod state;

#[tauri::command]
#[tracing::instrument(skip_all)]
fn refresh_serial_ports(app: AppHandle) -> Result<(), AppError> {
    refresh_serial_ports_intern(&app)
}

fn refresh_serial_ports_intern(app: &AppHandle) -> Result<(), AppError> {
    tracing::info!("Refreshing serial ports");

    let ports = crate::serial::available_port_models()?;

    app.emit_all("serial_ports_event", &ports)?;

    Ok(())
}

#[tauri::command]
#[tracing::instrument(skip_all)]
fn do_error() -> Result<(), AppError> {
    return Err(anyhow::anyhow!("Oops!").into());
}

pub fn run() -> anyhow::Result<()> {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle_creation = app.app_handle().clone();
            let app_handle_deletion = app.app_handle().clone();

            std::thread::spawn(move || {
                tracing::debug!("Starting serial creation events watcher");

                let watcher = SerialWatcher::new()?;
                let creation_iter = watcher.creation_iter()?;

                for serial_port in creation_iter {
                    let Ok(serial_port) = serial_port else {
                        break;
                    };

                    tracing::trace!(name=%serial_port.name(), "Serial creation event detected");

                    let _ = refresh_serial_ports_intern(&app_handle_creation);
                }

                tracing::debug!("Serial creation events watcher terminated");

                anyhow::Result::<()>::Ok(())
            });

            std::thread::spawn(move || {
                tracing::debug!("Starting serial deletion events watcher");

                let watcher = SerialWatcher::new()?;
                let deletion_iter = watcher.deletion_iter()?;

                for serial_port in deletion_iter {
                    let Ok(serial_port) = serial_port else {
                        break;
                    };

                    tracing::trace!(name=%serial_port.name(), "Serial deletion event detected");

                    let _ = refresh_serial_ports_intern(&app_handle_deletion);
                }

                tracing::debug!("Serial deletion events watcher terminated");

                anyhow::Result::<()>::Ok(())
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![refresh_serial_ports, do_error])
        .run(tauri::generate_context!())
        .context("Error while running tauri application")
}
