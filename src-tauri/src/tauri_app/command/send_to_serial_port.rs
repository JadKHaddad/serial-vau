use tokio_util::bytes::Bytes;

use crate::core::state::{
    open_serial_port::{CoreOutgoingPacket, CorePacketOrigin, SendError},
    State,
};

pub async fn send_to_serial_port_intern(
    name: String,
    bytes: Bytes,
    state: &State,
) -> Result<(), SendToSerialPortError> {
    tracing::info!(name=%name, "Sending to serial port");

    let packet = CoreOutgoingPacket {
        bytes,
        packet_origin: CorePacketOrigin::Direct,
    };

    Ok(state
        .send_to_open_serial_port(&name, packet)
        .await
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
