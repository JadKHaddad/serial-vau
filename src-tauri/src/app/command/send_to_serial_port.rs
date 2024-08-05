use tokio_util::bytes::Bytes;

use crate::core::state::{
    open_serial_port::{OutgoingPacket, PacketOrigin, SendError},
    AppState,
};

pub fn send_to_serial_port_intern(
    name: String,
    data: Bytes,
    state: &AppState,
) -> Result<(), SendToSerialPortError> {
    tracing::info!(name=%name, "Sending to serial port");

    let packet = OutgoingPacket::new_with_current_timestamp(data, PacketOrigin::Direct);

    Ok(state
        .send_to_open_serial_port(&name, packet)
        .ok_or(SendToSerialPortError::NotOpen)??)
}

#[derive(Debug, thiserror::Error)]
pub enum SendToSerialPortError {
    #[error("Port not open")]
    NotOpen,
    #[error("Failed to send: {0}")]
    SendError(
        #[source]
        #[from]
        SendError,
    ),
}
