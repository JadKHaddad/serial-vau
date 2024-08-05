use serde::Serialize;

use crate::app::model::incoming_packet::IncomingPacket as IncomingPacketModel;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IncomingPacket {
    #[serde(flatten)]
    pub incoming_packet: IncomingPacketModel,
}
