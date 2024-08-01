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

impl ManagedSerialPort {
    pub fn is_open(&self) -> bool {
        matches!(self.status, Status::Open)
    }

    pub fn is_closed(&self) -> bool {
        matches!(self.status, Status::Closed)
    }
}
