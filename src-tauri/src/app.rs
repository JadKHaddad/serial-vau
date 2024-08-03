use anyhow::Context;
use command::{
    close_serial_port::close_serial_port_intern,
    open_serial_port::{open_serial_port_intern, OpenSerialPortOptions},
    refresh_serial_ports::refresh_serial_ports_intern,
    send_to_all_serial_ports::send_to_all_serial_ports_intern,
    send_to_serial_port::send_to_serial_port_intern,
    subscribe::{subscribe_intern, unsubscribe_intern},
    toggle_read_state::toggle_read_state_intern,
};
use error::AppError;
use state::AppState;
use tauri::{AppHandle, Manager, State};

use crate::serial::watcher::Watcher as SerialWatcher;

mod command;
mod error;
mod model;
mod state;

#[tauri::command]
#[tracing::instrument(skip_all)]
pub fn refresh_serial_ports(app: AppHandle, state: State<AppState>) -> Result<(), AppError> {
    refresh_serial_ports_intern(&app, &state)
}

#[tauri::command]
#[tracing::instrument(skip_all)]
pub async fn open_serial_port(
    options: OpenSerialPortOptions,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    open_serial_port_intern(options, &state).await?;
    refresh_serial_ports_intern(&app, &state)?;

    Ok(())
}

#[tauri::command]
#[tracing::instrument(skip_all)]
pub async fn close_serial_port(
    name: String,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    close_serial_port_intern(name, &state).await?;
    refresh_serial_ports_intern(&app, &state)?;

    Ok(())
}

#[tauri::command]
#[tracing::instrument(skip_all)]
pub fn send_to_serial_port(
    name: String,
    value: String,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    let value = value.into();

    send_to_serial_port_intern(name, value, &state)?;

    Ok(())
}

#[tauri::command]
#[tracing::instrument(skip_all)]
pub fn send_to_all_serial_ports(value: String, state: State<'_, AppState>) {
    let value = value.into();

    send_to_all_serial_ports_intern(value, &state);
}

#[tauri::command]
#[tracing::instrument(skip_all)]
pub fn subscribe(
    from: &str,
    to: &str,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    subscribe_intern(from, to, &state)?;
    refresh_serial_ports_intern(&app, &state)?;

    Ok(())
}

#[tauri::command]
#[tracing::instrument(skip_all)]
pub fn unsubscribe(
    from: &str,
    to: &str,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    unsubscribe_intern(from, to, &state)?;
    refresh_serial_ports_intern(&app, &state)?;

    Ok(())
}

#[tauri::command]
#[tracing::instrument(skip_all)]
pub fn toggle_read_state(
    name: &str,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    toggle_read_state_intern(name, &state)?;
    refresh_serial_ports_intern(&app, &state)?;

    Ok(())
}

#[tauri::command]
#[tracing::instrument(skip_all)]
fn do_error() -> Result<(), AppError> {
    return Err(anyhow::anyhow!("Oops!").into());
}

/// Using [`thread`](std::thread) instead of `async tasks` to watch `serial` events,
/// because [`WMIConnection`](wmi::WMIConnection) is not [`Send`],
/// which is used in [`Watcher`](crate::serial::watcher::Watcher),
pub fn run() -> anyhow::Result<()> {
    let state = AppState::default();
    let state_creation = state.clone();
    let state_deletion = state.clone();

    tauri::Builder::default()
        .manage(state)
        .setup(|app| {
            let app_handle_creation = app.app_handle().clone();
            let app_handle_deletion = app.app_handle().clone();

            // See function's docs
            std::thread::spawn(move || {
                tracing::debug!("Starting serial creation events watcher");

                let watcher = SerialWatcher::new()?;
                let creation_iter = watcher.creation_iter()?;

                for serial_port in creation_iter {
                    match serial_port  {
                        Ok(serial_port) => {
                            tracing::trace!(name=%serial_port.name(), "Serial creation event detected");
                        },
                        Err(err) => {tracing::warn!(%err, "Serial creation event error");}
                    }

                    let _ = refresh_serial_ports_intern(&app_handle_creation, &state_creation);
                }

                tracing::debug!("Serial creation events watcher terminated");

                anyhow::Result::<()>::Ok(())
            });

            // See function's docs
            std::thread::spawn(move || {
                tracing::debug!("Starting serial deletion events watcher");

                let watcher = SerialWatcher::new()?;
                let deletion_iter = watcher.deletion_iter()?;

                for serial_port in deletion_iter {
                    match serial_port  {
                        Ok(serial_port) => {
                            tracing::trace!(name=%serial_port.name(), "Serial deletion event detected");
                        },
                        Err(err) =>{
                            tracing::warn!(%err, "Serial deletion event error");
                        }
                    }

                    let _ = refresh_serial_ports_intern(&app_handle_deletion, &state_deletion);
                }

                tracing::debug!("Serial deletion events watcher terminated");

                anyhow::Result::<()>::Ok(())
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            refresh_serial_ports,
            open_serial_port,
            close_serial_port,
            send_to_serial_port,
            send_to_all_serial_ports,
            subscribe,
            unsubscribe,
            toggle_read_state,
            do_error
        ])
        .run(tauri::generate_context!())
        .context("Error while running tauri application")
}
