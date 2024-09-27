use tokio_util::bytes::Bytes;

use crate::core::state::{
    open_serial_port::{OutgoingPacket, PacketOrigin},
    State,
};

pub async fn send_to_all_serial_ports_intern(bytes: Bytes, state: &State) {
    tracing::info!("Sending to all serial ports");

    let packet = OutgoingPacket {
        bytes,
        packet_origin: PacketOrigin::Broadcast,
    };

    state.send_to_all_open_serial_ports(packet).await
}
