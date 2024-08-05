use tauri::{AppHandle, Manager};

use crate::{
    app::{
        event::incoming_packet::IncomingPacket as IncomingPacketEvent,
        model::{incoming_packet::IncomingPacket, open_options::OpenSerialPortOptions},
    },
    core::state::{error::OpenSerialPortError, AppState},
};

pub async fn open_serial_port_intern(
    options: OpenSerialPortOptions,
    app: &AppHandle,
    state: &AppState,
) -> Result<(), OpenSerialPortError> {
    tracing::info!(?options, "Opening serial port");

    let incoming_name = options.name.clone();
    let outgoing_name = options.name.clone();

    let (mut incoming_rx, mut outgoing_tx) = state.open_serial_port(options.into()).await?;

    let app_handle_read = app.clone();
    tokio::spawn(async move {
        while let Some(packet) = incoming_rx.recv().await {
            match packet {
                Ok(packet) => {
                    let incoming_packet_event = IncomingPacketEvent {
                        incoming_packet: IncomingPacket {
                            from: incoming_name.clone(),
                            timestamp_millis: packet.timestamp_millis,
                            line: packet.line,
                        },
                    };

                    let _ = app_handle_read.emit_all("serial_line_event", &incoming_packet_event);
                }
                Err(err) => {
                    tracing::error!(%err, from=%incoming_name, "Error receiving data");
                }
            }
        }
    });

    tokio::spawn(async move {
        while let Some(packet) = outgoing_tx.recv().await {
            match packet {
                Ok(packet) => {
                    // TODO: Send outgoing packet event!
                }
                Err(err) => {
                    tracing::error!(%err, to=%outgoing_name, "Error sending data");
                }
            }
        }
    });

    Ok(())
}
