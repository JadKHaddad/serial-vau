use tauri::{AppHandle, Manager};

use crate::{
    app::{
        event::packet::PacketEvent,
        model::{
            open_options::OpenSerialPortOptions,
            packet::{IncomingPacket, OutgoingPacket, Packet, PacketDirection},
        },
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
                    // TODO: consider implementing from core data types to model/packet data types
                    let event = PacketEvent {
                        packet: Packet {
                            packet_direction: PacketDirection::Incoming(IncomingPacket {
                                line: packet.line,
                            }),
                            port_name: incoming_name.clone(),
                            timestamp_millis: packet.timestamp_millis,
                        },
                    };

                    let _ = app_handle_read.emit_all("serial_packet_event", &event);
                }
                Err(err) => {
                    tracing::error!(%err, from=%incoming_name, "Error receiving data");
                }
            }
        }
    });

    let app_handle_write = app.clone();
    tokio::spawn(async move {
        while let Some(packet) = outgoing_tx.recv().await {
            match packet {
                Ok(packet) => {
                    // TODO: consider implementing from core data types to model/packet data types
                    let event = PacketEvent {
                        packet: Packet {
                            packet_direction: PacketDirection::Outgoing(OutgoingPacket {
                                packet_origin: packet.origin.into(),
                                data: packet.data.to_vec(),
                            }),
                            port_name: outgoing_name.clone(),
                            timestamp_millis: packet.timestamp_millis,
                        },
                    };

                    let _ = app_handle_write.emit_all("serial_packet_event", &event);
                }
                Err(err) => {
                    tracing::error!(%err, to=%outgoing_name, "Error sending data");
                }
            }
        }
    });

    Ok(())
}
