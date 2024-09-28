/// Defines if an open serial port is currently reading or stopped.
#[derive(Debug, Clone, Copy, Default)]
pub enum CoreReadState {
    #[default]
    Read,
    Stop,
}

/// Defines additional information if the port is in [`Status::Open`] state.
#[derive(Debug)]
pub struct CoreOpenStatus {
    pub read_state: CoreReadState,
}

/// Status of a serial port.
#[derive(Debug)]
pub enum Status {
    Closed,
    Open(CoreOpenStatus),
}

impl CoreReadState {
    pub fn is_stop(&self) -> bool {
        matches!(self, Self::Stop)
    }

    pub fn toggle(self) -> Self {
        match self {
            Self::Read => Self::Stop,
            Self::Stop => Self::Read,
        }
    }
}

#[derive(Debug)]
pub struct CoreManagedSerialPort {
    pub name: String,
    pub status: Status,
    #[cfg(feature = "subscriptions")]
    pub subscriptions: Vec<String>,
    #[cfg(feature = "subscriptions")]
    pub subscribed_to: Vec<String>,
}

impl CoreManagedSerialPort {
    pub fn is_open(&self) -> bool {
        matches!(self.status, Status::Open(_))
    }

    pub fn is_closed(&self) -> bool {
        matches!(self.status, Status::Closed)
    }
}
