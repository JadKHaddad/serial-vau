use tauri::AppHandle;

use crate::app::state::{open_serial_port::OpenSerialPortOptions, AppState, OpenSerialPortError};

pub async fn open_serial_port_intern(
    options: OpenSerialPortOptions,
    app: &AppHandle,
    state: &AppState,
) -> Result<(), OpenSerialPortError> {
    tracing::info!(?options, "Opening serial port");

    let incoming_name = options.name.clone();
    let outgoing_name = options.name.clone();

    let (mut incoming_rx, mut outgoing_tx) = state.open_serial_port(options).await?;

    tokio::spawn(async move {
        while let Some(packet) = incoming_rx.recv().await {
            match packet {
                Ok(packet) => {
                    // TODO: Send incoming packet event!
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
