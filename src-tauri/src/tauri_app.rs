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
use futures::StreamExt;
use error::AppError;
use event::emit_managed_serial_ports::emit_managed_serial_ports;
use model::{managed_serial_port::ManagedSerialPort, open_options::OpenSerialPortOptions};
use state::TauriAppState as TauriAppState;
use tauri::{AppHandle, Manager, State};

use crate::{app::{database::database_impl::sqlite_database_service::SqliteDatabase, state::AppState}, serial_manager::serial_manager_impl::tokio_serial_manager::TokioSerialManager, watcher::{model::WatcherEventType, watcher_impl::watcher::WatcherImpl, watcher_service::WatcherService, Watcher}};

mod command;
mod error;
mod event;
mod model;
mod state;

#[tauri::command]
#[tracing::instrument(skip_all)]
pub async fn get_serial_ports(
    state: State<'_, TauriAppState>,
) -> Result<Vec<ManagedSerialPort>, AppError> {
    get_serial_ports_intern(&state)
        .await
        .map_err(Into::into)
}

#[tauri::command]
#[tracing::instrument(skip_all)]
pub async fn open_serial_port(
    name: String,
    options: OpenSerialPortOptions,
    app: AppHandle,
    state: State<'_, TauriAppState>,
) -> Result<Vec<ManagedSerialPort>, AppError> {
    open_serial_port_intern(name, options, &app, &state)
        .await
        .map_err(Into::into)
}

#[tauri::command]
#[tracing::instrument(skip_all)]
pub async fn close_serial_port(
    name: String,
    state: State<'_, TauriAppState>,
) -> Result<Vec<ManagedSerialPort>, AppError> {
    close_serial_port_intern(name, &state)
        .await
        .map_err(Into::into)
}

#[tauri::command]
#[tracing::instrument(skip_all)]
pub async fn send_to_serial_port(
    name: String,
    value: String,
    state: State<'_, TauriAppState>,
) -> Result<(), AppError> {
    send_to_serial_port_intern(name, value.into(), state.serial_state())
        .await
        .map_err(Into::into)
}

#[tauri::command]
#[tracing::instrument(skip_all)]
pub async fn send_to_all_serial_ports(
    value: String,
    state: State<'_, TauriAppState>,
) -> Result<(), AppError> {
    send_to_all_serial_ports_intern(value.into(), state.serial_state()).await;
    Ok(())
}

#[tauri::command]
#[tracing::instrument(skip_all)]
pub async fn subscribe(
    from: &str,
    to: &str,
    state: State<'_, TauriAppState>,
) -> Result<Vec<ManagedSerialPort>, AppError> {
    subscribe_intern(from, to, &state)
        .await
        .map_err(Into::into)
}

#[tauri::command]
#[tracing::instrument(skip_all)]
pub async fn unsubscribe(
    from: &str,
    to: &str,
    state: State<'_, TauriAppState>,
) -> Result<Vec<ManagedSerialPort>, AppError> {
    unsubscribe_intern(from, to, &state)
        .await
        .map_err(Into::into)
}

#[tauri::command]
#[tracing::instrument(skip_all)]
pub async fn toggle_read_state(
    name: &str,
    state: State<'_, TauriAppState>,
) -> Result<Vec<ManagedSerialPort>, AppError> {
    toggle_read_state_intern(name, &state)
        .await
        .map_err(Into::into)
}

#[tauri::command]
#[tracing::instrument(skip_all)]
fn do_error() -> Result<(), AppError> {
    return Err(anyhow::anyhow!("Oops!").into());
}

pub fn run() -> anyhow::Result<()> {
    let serial_manager = TokioSerialManager::new();

    // TODO: run the migrations!
    // We might need to create the sqlite database file first if (sqlite connection) and it doesn't exist.
    let db = tauri::async_runtime::block_on(SqliteDatabase::new("sqlite:../sqlite.db"))?;


    // TODO: Use in setup!
    let app_state = AppState::new(db.into(), serial_manager.into());

    let tauri_app_state = TauriAppState::new(app_state);
    
    let tauri_app_state_wachter = tauri_app_state.clone();
    tauri::Builder::default()
        .manage(tauri_app_state)
        .setup(|app| {
                let app_handle = app.app_handle().clone();
                tauri::async_runtime::spawn(async move {
                    let pool = tokio_util::task::LocalPoolHandle::new(1);

                    let _ = pool
                        .spawn_pinned(|| async move {
                            let watcher: Watcher = WatcherImpl::new()?.into();
                        
                            let mut stream = std::pin::pin!(watcher.events_stream()?);

                            tracing::debug!("Starting serial events watcher");

                            while let Some(event) = stream.next().await {
                                match event {
                                    Err(err) => {
                                        tracing::warn!(%err, "Serial event error");
                                        
                                        // TODO: Emit error and break

                                        break;
                                    }
                                    Ok(event) => match event.event_type {
                                        WatcherEventType::Creation => {
                                            tracing::trace!(name=%event.serial_port.name(), "Serial creation event detected");
                                        }
                                        WatcherEventType::Deletion => {
                                            tracing::trace!(name=%event.serial_port.name(), "Serial deletion event detected");
                                        }
                                    },
                                }

                                let _ = emit_managed_serial_ports(&app_handle, &tauri_app_state_wachter).await;
                            }

                            tracing::debug!("Serial events watcher terminated");

                            anyhow::Result::<()>::Ok(())
                        })
                        .await;
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
