use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum Status {
    Closed,
    Open,
}

#[derive(Debug, Serialize)]
pub struct ManagedSerialPort {
    pub name: String,
    pub status: Status,
}
