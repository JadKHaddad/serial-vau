use anyhow::Context;
use command::{
    close_serial_port::close_serial_port_intern,
    get_serial_ports::get_serial_ports_intern,
    open_serial_port::open_serial_port_intern,
    send_to_all_serial_ports::send_to_all_serial_ports_intern,
    send_to_serial_port::send_to_serial_port_intern,
    subscribe::{subscribe_intern, unsubscribe_intern},
    toggle_read_state::toggle_read_state_intern,
};
use error::AppError;
use event::emit_managed_serial_ports::emit_managed_serial_ports;
use model::{managed_serial_port::ManagedSerialPort, open_options::OpenSerialPortOptions};
use state::State as TauriAppState;
use tauri::{AppHandle, Manager, State};

use crate::app::state::State as AppState;
use crate::core::{serial::watcher::Watcher as SerialWatcher, state::State as SerialState};

mod command;
mod error;
mod event;
mod model;
mod state;

#[tauri::command]
#[tracing::instrument(skip_all)]
pub fn get_serial_ports(state: State<TauriAppState>) -> Result<Vec<ManagedSerialPort>, AppError> {
    get_serial_ports_intern(state.serial_state()).map_err(Into::into)
}

#[tauri::command]
#[tracing::instrument(skip_all)]
pub async fn open_serial_port(
    options: OpenSerialPortOptions,
    app: AppHandle,
    state: State<'_, TauriAppState>,
) -> Result<Vec<ManagedSerialPort>, AppError> {
    open_serial_port_intern(options, &app, &state)
        .await
        .map_err(Into::into)
}

#[tauri::command]
#[tracing::instrument(skip_all)]
pub async fn close_serial_port(
    name: String,
    state: State<'_, TauriAppState>,
) -> Result<Vec<ManagedSerialPort>, AppError> {
    close_serial_port_intern(name, state.serial_state())
        .await
        .map_err(Into::into)
}

#[tauri::command]
#[tracing::instrument(skip_all)]
pub fn send_to_serial_port(
    name: String,
    value: String,
    state: State<'_, TauriAppState>,
) -> Result<(), AppError> {
    send_to_serial_port_intern(name, value.into(), state.serial_state()).map_err(Into::into)
}

#[tauri::command]
#[tracing::instrument(skip_all)]
pub fn send_to_all_serial_ports(value: String, state: State<'_, TauriAppState>) {
    send_to_all_serial_ports_intern(value.into(), state.serial_state());
}

#[tauri::command]
#[tracing::instrument(skip_all)]
pub fn subscribe(
    from: &str,
    to: &str,
    state: State<'_, TauriAppState>,
) -> Result<Vec<ManagedSerialPort>, AppError> {
    subscribe_intern(from, to, state.serial_state()).map_err(Into::into)
}

#[tauri::command]
#[tracing::instrument(skip_all)]
pub fn unsubscribe(
    from: &str,
    to: &str,
    state: State<'_, TauriAppState>,
) -> Result<Vec<ManagedSerialPort>, AppError> {
    unsubscribe_intern(from, to, state.serial_state()).map_err(Into::into)
}

#[tauri::command]
#[tracing::instrument(skip_all)]
pub fn toggle_read_state(
    name: &str,
    state: State<'_, TauriAppState>,
) -> Result<Vec<ManagedSerialPort>, AppError> {
    toggle_read_state_intern(name, state.serial_state()).map_err(Into::into)
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
    let serial_state = SerialState::default();
    let serial_state_creation = serial_state.clone();
    let serial_state_deletion = serial_state.clone();

    let app_state = AppState::default();

    let tauri_app_state = TauriAppState::new(serial_state, app_state);
    tauri::Builder::default()
        .manage(tauri_app_state)
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

                    let _ = emit_managed_serial_ports(&app_handle_creation, &serial_state_creation);
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

                    let _ = emit_managed_serial_ports(&app_handle_deletion, &serial_state_deletion);
                }

                tracing::debug!("Serial deletion events watcher terminated");

                anyhow::Result::<()>::Ok(())
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_serial_ports,
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
