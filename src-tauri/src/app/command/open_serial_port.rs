use futures::StreamExt;
use serde::Deserialize;
use tauri::async_runtime;
use tokio::io::AsyncWriteExt;
use tokio_serial::{DataBits, FlowControl, Parity, SerialPortBuilderExt, StopBits};
use tokio_util::{
    codec::{FramedRead, LinesCodec},
    sync::CancellationToken,
};

use crate::{
    app::{
        open_serial_port::OpenSerialPort,
        state::{AppState, ManagedSerialPortsError},
    },
    serial::SerialPort,
};

#[derive(Debug, Deserialize)]
pub struct OpenSerialPortOptions {
    name: String,
}

pub async fn open_serial_port_intern(
    options: OpenSerialPortOptions,
    state: &AppState,
) -> Result<(), OpenSerialPortError> {
    tracing::info!(?options, "Opening serial port");

    let port_to_open_name = state
        .is_port_closed(&options.name)?
        .ok_or(OpenSerialPortError::NotFound)?
        .then_some(&options.name)
        .ok_or(OpenSerialPortError::AlreadyOpen)?;

    let port = tokio_serial::new(port_to_open_name, 115200)
        .stop_bits(StopBits::One)
        .data_bits(DataBits::Eight)
        .flow_control(FlowControl::None)
        .parity(Parity::None)
        .open_native_async()?;

    let (port_read, mut port_write) = tokio::io::split(port);
    let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(100);
    let cancellation_token = CancellationToken::new();

    let mut framed_read_lines_port = FramedRead::new(port_read, LinesCodec::new());

    state.add_open_serial_port(OpenSerialPort::new(
        SerialPort::new(options.name.clone()),
        tx,
        cancellation_token.clone(),
    ));

    let read_state = state.clone();
    let read_cancellation_token = cancellation_token;
    let read_name = options.name.clone();
    async_runtime::spawn(async move {
        loop {
            tokio::select! {
                line = framed_read_lines_port.next() => {
                    match line {
                        Some(Ok(line)) => {
                            tracing::trace!(name=%read_name, %line, "Received");
                        }
                        Some(Err(err)) => {
                            tracing::error!(name=%read_name, %err);

                            tracing::debug!(name=%read_name, "Removing serial port due to an error");
                            read_state.remove_open_serial_port(&read_name);

                            break;
                        }
                        _ => {}
                    }
                },
                _ = read_cancellation_token.cancelled() => {
                    tracing::debug!(name=%read_name, "Cancelled");

                    break;
                }
            }
        }

        tracing::debug!(name=%read_name, "Read task terminated")
    });

    let write_name = options.name.clone();
    async_runtime::spawn(async move {
        while let Some(value) = rx.recv().await {
            tracing::trace!(name=%write_name, %value, "Sending");

            match port_write.write_all(value.as_bytes()).await {
                Ok(_) => {}
                Err(err) => {
                    tracing::error!(name=%write_name, %err);
                    break;
                }
            }
        }

        tracing::debug!(name=%write_name, "Write task terminated")
    });

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum OpenSerialPortError {
    #[error("Failed to get managed ports: {0}")]
    ManagedSerialPortsError(
        #[source]
        #[from]
        ManagedSerialPortsError,
    ),
    #[error("Port not found")]
    NotFound,
    #[error("Port already open")]
    AlreadyOpen,
    #[error("Failed to open port: {0}")]
    FailedToOpen(
        #[source]
        #[from]
        tokio_serial::Error,
    ),
}
