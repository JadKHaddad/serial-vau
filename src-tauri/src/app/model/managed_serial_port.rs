use serde::Serialize;

use crate::app::state::open_serial_port::ReadState;

#[derive(Debug, Serialize)]
pub enum Status {
    Closed,
    Open,
}

#[derive(Debug, Serialize)]
pub struct ManagedSerialPort {
    pub name: String,
    pub status: Status,
    #[cfg(feature = "subscriptions")]
    pub subscriptions: Vec<String>,
    #[cfg(feature = "subscriptions")]
    pub subscribed_to: Vec<String>,
    // TODO: remove the option, and move read_state to Status::Open
    pub read_state: Option<ReadState>,
}

impl ManagedSerialPort {
    pub fn is_open(&self) -> bool {
        matches!(self.status, Status::Open)
    }

    pub fn is_closed(&self) -> bool {
        matches!(self.status, Status::Closed)
    }
}
