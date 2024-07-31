use parking_lot::RwLock;
use tokio_util::sync::CancellationToken;

use crate::serial::SerialPortModel;

#[derive(Debug)]
pub struct OpenSerialPort {
    serial_port: SerialPortModel,
    cancellation_token: CancellationToken,
}

#[derive(Debug, Default)]
pub struct AppState {
    open_serial_ports: RwLock<Vec<OpenSerialPort>>,
}

impl AppState {
    pub fn new() -> Self {
        Default::default()
    }
}
