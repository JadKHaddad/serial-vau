use tauri::AppHandle;

use crate::{
    app::{
        model::managed_serial_port::AppOpenSerialPortOptions,
        serial_state::error::{CoreIncomingPacketError, CorePacketError},
        state::error::{AppManagedSerialPortsError, AppOpenSerialPortError, AppPacketError},
    },
    tauri_app::{
        event::{
            emit_error::emit_error_event,
            emit_managed_serial_ports::emit_managed_serial_ports_event,
            emit_packet::emit_packet_event,
            model::{error::ErrorEvent, packet::PacketEvent},
        },
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

    let mut rx = state
        .app_state()
        .open_serial_port(&name, app_options)
        .await?;

    let app = app.clone();

    let tauri_app_state = state.clone();
    tokio::spawn(async move {
        tracing::debug!(name=%name, "Read events task started");

        while let Some(packet) = rx.recv().await {
            match packet {
                Ok(packet) => {
                    let event = PacketEvent {
                        packet: packet.into(),
                    };

                    let _ = emit_packet_event(&app, &event);
                }
                Err(err) => {
                    tracing::error!(%err, from=%name, "Error receiving data");

                    // The watcher should detect if the port was closed and notify the ui.
                    // Emit changed to the ui. The Error may be due to the port being closed.
                    // Or the device may have not been detected by the watcher.

                    match err {
                        // Decoding lines error will not break the read loop in `State.open_serial_port`.
                        AppPacketError::CorePacketError(CorePacketError::Incoming(
                            CoreIncomingPacketError::Codec(..),
                        )) => {}
                        // Save packet error will not break the read loop in `State.open_serial_port`.
                        AppPacketError::SavePacketError(..) => {}
                        _ => {
                            let _ = emit_managed_serial_ports_event(&app, &tauri_app_state).await;
                        }
                    }

                    let error_event = ErrorEvent::from(err);

                    let _ = emit_error_event(&app, &error_event);
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
        AppOpenSerialPortError,
    ),
    #[error("Failed to get managed ports: {0}")]
    ManagedSerialPortsError(
        #[source]
        #[from]
        AppManagedSerialPortsError,
    ),
}
