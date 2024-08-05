use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IncomingPacket {
    pub from: String,
    pub line: String,
    pub timestamp_millis: u64,
}
