use tokio::io::{AsyncRead, AsyncWrite};

use crate::serial_manager::{
    error::{SerialManagerAvailablePortsError, SerialManagerOpenPortError},
    models::{SerialManagerOpenSerialPortOptions, SerialManagerPort},
    serial_manager_service::SerialManagerService,
};

#[derive(Debug)]
pub struct DummySerialManager;

impl SerialManagerService for DummySerialManager {
    fn available_ports(&self) -> Result<Vec<SerialManagerPort>, SerialManagerAvailablePortsError> {
        Ok(Vec::new())
    }

    fn open_port(
        &self,
        _name: &str,
        _options: SerialManagerOpenSerialPortOptions,
    ) -> Result<impl AsyncRead + AsyncWrite, SerialManagerOpenPortError> {
        Ok(tokio::io::duplex(1024).0)
    }
}
