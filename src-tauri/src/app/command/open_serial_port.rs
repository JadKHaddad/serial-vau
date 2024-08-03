use futures::{SinkExt, StreamExt};
use serde::Deserialize;
use tokio_serial::{DataBits, FlowControl, Parity, SerialPortBuilderExt, StopBits};
use tokio_util::{
    bytes::{Bytes, BytesMut},
    codec::{BytesCodec, Decoder, FramedRead, FramedWrite, LinesCodec},
    sync::CancellationToken,
};

use crate::{
    app::state::{open_serial_port::OpenSerialPort, AppState, ManagedSerialPortsError},
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

    let (port_read, port_write) = tokio::io::split(port);
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<Bytes>();
    let cancellation_token = CancellationToken::new();

    let mut framed_read_bytes_port = FramedRead::new(port_read, BytesCodec::new());
    let mut framed_write_bytes_port = FramedWrite::new(port_write, BytesCodec::new());

    state.add_open_serial_port(OpenSerialPort::new(
        SerialPort::new(options.name.clone()),
        tx,
        cancellation_token.clone(),
    ));

    let subscriptions = state.subscriptions();
    let read_state = state.clone();
    let read_cancellation_token = cancellation_token.clone();
    let read_name = options.name.clone();

    tokio::spawn(async move {
        let mut lines_codec = LinesCodec::new();
        let mut lines_bytes = BytesMut::new();

        loop {
            tokio::select! {
                bytes = framed_read_bytes_port.next() => {
                    match bytes {
                        Some(Ok(bytes)) => {
                            tracing::trace!(target: "serial_vau::serial::read::byte", name=%read_name, ?bytes, "Read");

                            if let Some( subscriptions) = subscriptions.read().get(&read_name){
                                for (subscriber_name, tx_handle) in subscriptions {
                                    if let Some(tx_handle) = tx_handle {
                                        tracing::trace!(target: "serial_vau::serial::read::byte::subscribe", name=%read_name, subscriber=%subscriber_name, "Sending bytes to subscriber");
                                        if let Err(err) = tx_handle.send(bytes.clone().into()) {
                                            tracing::error!(target: "serial_vau::serial::read::byte::subscribe", name=%read_name, subscriber=%subscriber_name, %err, "Failed to send bytes to subscriber");
                                        }
                                    }
                                }
                            }

                            lines_bytes.extend_from_slice(&bytes);

                            loop {
                                match lines_codec.decode(&mut lines_bytes) {
                                    Ok(None) => break,
                                    Ok(Some(line)) => {
                                        tracing::trace!(target: "serial_vau::serial::read::line", name=%read_name, %line, "Read");
                                    }
                                    Err(err) => {
                                        tracing::warn!(target: "serial_vau::serial::read::line", name=%read_name, %err, "Failed to decode line");

                                        // Clear the buffer to prevent further errors.
                                        lines_bytes.clear();

                                        break;
                                    }
                                }
                            }
                        }
                        Some(Err(err)) => {
                            tracing::error!(target: "serial_vau::serial::read", name=%read_name, %err);

                            // Removing the port will drop the sender causing the write loop to break.
                            tracing::debug!(target: "serial_vau::serial::read", name=%read_name, "Removing serial port due to an error");
                            read_state.remove_open_serial_port(&read_name);

                            break;
                        }
                        _ => {}
                    }
                },
                _ = read_cancellation_token.cancelled() => {
                    // At this point we should have been removed and cancelled. Nothing to do here.
                    tracing::debug!(target: "serial_vau::serial::read", name=%read_name, "Cancelled");

                    break;
                }
            }
        }

        tracing::debug!(target: "serial_vau::serial::read", name=%read_name, "Read task terminated")
    });

    let write_name = options.name.clone();
    let write_cancellation_token = cancellation_token;

    tokio::spawn(async move {
        // Dropping the sender will automatically break the loop.
        while let Some(value) = rx.recv().await {
            tracing::trace!(target: "serial_vau::serial::write::byte", name=%write_name, value=?value, "Sending");
            tracing::trace!(target: "serial_vau::serial::write::string", name=%write_name, value=%String::from_utf8_lossy(&value), "Sending");

            tokio::select! {
                send_result = framed_write_bytes_port.send(value) => {
                    match send_result {
                        Ok(_) => {
                            tracing::trace!(target: "serial_vau::serial::write::result", "Ok");
                        }
                        Err(err) => {
                            // If the write fails we just break out of the loop.
                            // Read task must have also been terminated due to the same error.
                            tracing::error!(target: "serial_vau::serial::write::result", name=%write_name, %err);

                            break;
                        }
                    }
                },
                _ = write_cancellation_token.cancelled() => {
                    tracing::debug!(target: "serial_vau::serial::write::result", name=%write_name, "Cancelled");

                    break;
                }
            }
        }

        tracing::debug!(target: "serial_vau::serial::write", name=%write_name, "Write task terminated")
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
