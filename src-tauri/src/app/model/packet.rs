use crate::core::state::open_serial_port::CorePacketDirection;

#[derive(Debug, Default)]
pub struct AppPacket {
    pub packet_direction: CorePacketDirection, // TODO: PacketDirection should be a database model and not come from the core.
    pub timestamp_millis: u64,
}
