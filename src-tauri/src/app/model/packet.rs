use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IncomingPacket {
    pub line: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionPacketOrigin {
    pub name: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type", content = "content")]
pub enum PacketOrigin {
    Direct,
    Broadcast,
    Subscription(SubscriptionPacketOrigin),
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OutgoingPacket {
    pub packet_origin: PacketOrigin,
    pub data: Vec<u8>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type", content = "content")]
pub enum PacketDirection {
    /// From the open serial port to the application
    Incoming(IncomingPacket),
    /// From the application to the open serial port
    Outgoing(OutgoingPacket),
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Packet {
    pub packet_direction: PacketDirection,
    pub port_name: String,
    pub timestamp_millis: u64,
}

mod core_impl {
    use super::*;
    use crate::core::state::open_serial_port::PacketOrigin as CorePacketOrigin;

    impl From<CorePacketOrigin> for PacketOrigin {
        fn from(value: CorePacketOrigin) -> Self {
            match value {
                CorePacketOrigin::Direct => Self::Direct,
                CorePacketOrigin::Broadcast => Self::Broadcast,
                CorePacketOrigin::Subscription { from } => {
                    Self::Subscription(SubscriptionPacketOrigin { name: from })
                }
            }
        }
    }
}
