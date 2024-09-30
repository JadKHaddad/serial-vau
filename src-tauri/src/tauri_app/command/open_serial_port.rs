use tauri::{AppHandle, Manager};

use crate::{
    app::{
        model::managed_serial_port::AppOpenSerialPortOptions, state::AppManagedSerialPortsError,
    },
    core::state::error::{CoreIncomingPacketError, CoreOpenSerialPortError, CorePacketError},
    tauri_app::{
        event::{emit_managed_serial_ports::emit_managed_serial_ports, model::packet::PacketEvent},
        model::{managed_serial_port::ManagedSerialPort, open_options::OpenSerialPortOptions},
        state::TauriAppState,
    },
};

pub async fn open_serial_port_intern(
    name: String,
    options: OpenSerialPortOptions,
    app: &AppHandle,
    state: &TauriAppState,
) -> Result<Vec<ManagedSerialPort>, OpenSerialPortError> {
    tracing::info!(?options, "Opening serial port");

    let app_options: AppOpenSerialPortOptions = options.into();

    // save the options
    if let Err(err) = state
        .app_state()
        .add_or_update_open_serial_port_options(&name, &app_options)
        .await
    {
        tracing::error!(%err, name=%name, "Error adding or updating open serial port options");

        // TODO: Emit non-fatal error
    }

    let mut rx = state
        .serial_state()
        .open_serial_port(&name, app_options.core_options)
        .await?;

    let app = app.clone();

    let tauri_app_state = state.clone();
    let app_state = state.app_state().clone();
    tokio::spawn(async move {
        tracing::debug!(name=%name, "Read events task started");

        while let Some(packet) = rx.recv().await {
            match packet {
                Ok(packet) => {
                    // Note: May not be needed. see `crate::app::state::State`
                    if let Err(err) = app_state.add_packet(&packet).await {
                        tracing::error!(%err, from=%name, "Error adding packet to app state");

                        // TODO: Emit non-fatal error
                    }

                    let event = PacketEvent {
                        packet: packet.into(),
                    };

                    let _ = app.emit_all("serial_packet_event", &event);
                }
                Err(err) => {
                    tracing::error!(%err, from=%name, "Error receiving data");

                    // The watcher should detect if the port was closed and notify the ui.
                    // Emit changed to the ui. The Error may be due to the port being closed.
                    // Or the device may have not been detected by the watcher.

                    match err {
                        // Decoding lines error will not break the read loop in `State.open_serial_port`.
                        CorePacketError::Incoming(CoreIncomingPacketError::Codec(..)) => {}
                        _ => {
                            let _ = emit_managed_serial_ports(&app, &tauri_app_state).await;
                        }
                    }
                }
            }
        }

        tracing::debug!(name=%name, "Read events task terminated");
    });

    let managed_serial_ports = state.get_managed_serial_ports().await?;

    Ok(managed_serial_ports)
}

#[derive(Debug, thiserror::Error)]
pub enum OpenSerialPortError {
    #[error("Failed to open serial port: {0}")]
    OpenSerialPortError(
        #[source]
        #[from]
        CoreOpenSerialPortError,
    ),
    #[error("Failed to get managed ports: {0}")]
    ManagedSerialPortsError(
        #[source]
        #[from]
        AppManagedSerialPortsError,
    ),
}
