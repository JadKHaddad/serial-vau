use tauri::{AppHandle, Manager};

use crate::{
    core::state::error::{ManagedSerialPortsError, OpenSerialPortError as CoreOpenSerialPortError},
    tauri_app::{
        event::model::packet::PacketEvent,
        model::{managed_serial_port::ManagedSerialPort, open_options::OpenSerialPortOptions},
        state::State as TauriAppState,
    },
};

pub async fn open_serial_port_intern(
    options: OpenSerialPortOptions,
    app: &AppHandle,
    state: &TauriAppState,
) -> Result<Vec<ManagedSerialPort>, OpenSerialPortError> {
    tracing::info!(?options, "Opening serial port");

    let name = options.name.clone();

    let mut rx = state
        .serial_state()
        .open_serial_port(options.into())
        .await?;

    let app = app.clone();
    let packets = state.app_state().get_or_create_packets(&name).await;

    tokio::spawn(async move {
        while let Some(packet) = rx.recv().await {
            match packet {
                Ok(packet) => {
                    // Note: May not be needed. see `crate::app::state::State`
                    packets.push(&packet).await;

                    let event = PacketEvent {
                        packet: packet.into(),
                    };

                    let _ = app.emit_all("serial_packet_event", &event);
                }
                Err(err) => {
                    tracing::error!(%err, from=%name, "Error receiving data");
                }
            }
        }
    });

    let managed_serial_ports = state.serial_state().managed_serial_ports().await?;
    let managed_serial_ports = managed_serial_ports.into_iter().map(Into::into).collect();

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
        ManagedSerialPortsError,
    ),
}
