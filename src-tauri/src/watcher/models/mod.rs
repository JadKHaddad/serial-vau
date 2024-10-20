#[derive(Debug)]
pub struct WatcherPort {
    name: String,
}

impl WatcherPort {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug)]
pub struct WatcherPortEvent {
    pub event_type: WatcherEventType,
    pub serial_port: WatcherPort,
}

#[derive(Debug)]
pub enum WatcherEventType {
    Creation,
    Deletion,
}
