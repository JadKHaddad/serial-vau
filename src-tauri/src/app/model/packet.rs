use crate::core::state::open_serial_port::PacketDirection;

#[derive(Debug)]
pub struct Packet {
    pub packet_direction: PacketDirection,
    pub timestamp_millis: u64,
}
