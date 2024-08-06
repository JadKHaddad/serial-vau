use tauri::{AppHandle, Manager};

use crate::{
    app::{event::packet::PacketEvent, model::open_options::OpenSerialPortOptions},
    core::state::{error::OpenSerialPortError, AppState},
};

pub async fn open_serial_port_intern(
    options: OpenSerialPortOptions,
    app: &AppHandle,
    state: &AppState,
) -> Result<(), OpenSerialPortError> {
    tracing::info!(?options, "Opening serial port");

    let name = options.name.clone();

    let mut rx = state.open_serial_port(options.into()).await?;

    let app = app.clone();
    tokio::spawn(async move {
        while let Some(packet) = rx.recv().await {
            match packet {
                Ok(packet) => {
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

    Ok(())
}
