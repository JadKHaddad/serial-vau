use tokio_util::bytes::Bytes;

use crate::app::state::{
    open_serial_port::{OutgoingPacket, PacketOrigin},
    AppState,
};

pub fn send_to_all_serial_ports_intern(data: Bytes, state: &AppState) {
    tracing::info!("Sending to all serial ports");

    let packet = OutgoingPacket::new_with_current_timestamp(data, PacketOrigin::Broadcast);

    state.send_to_all_open_serial_ports(packet)
}
