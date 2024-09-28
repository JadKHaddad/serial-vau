use crate::core::state::open_serial_port::PacketDirection;

#[derive(Debug, Default)]
pub struct Packet {
    pub packet_direction: PacketDirection, // TODO: PacketDirection should be a database model and not come from the core.
    pub timestamp_millis: u64,
}
