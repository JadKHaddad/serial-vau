use serde::Serialize;

use crate::app::model::packet::Packet;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PacketEvent {
    pub packet: Packet,
}
