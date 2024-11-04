use tokio::io::{AsyncRead, AsyncWrite};

use crate::serial_manager::{
    error::{SerialManagerAvailablePortsError, SerialManagerOpenPortError},
    model::{SerialManagerOpenSerialPortOptions, SerialManagerPort},
    serial_manager_service::SerialManagerService,
};

#[derive(Debug)]
pub struct DummySerialManager {
    _private: (),
}

impl DummySerialManager {
    pub fn new() -> Self {
        tracing::info!("Creating Dummy Serial Manager");

        Self { _private: () }
    }
}

impl Default for DummySerialManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SerialManagerService for DummySerialManager {
    fn available_ports(&self) -> Result<Vec<SerialManagerPort>, SerialManagerAvailablePortsError> {
        Ok(Vec::new())
    }

    fn open_port<'a>(
        &self,
        _name: impl Into<std::borrow::Cow<'a, str>>,
        _options: SerialManagerOpenSerialPortOptions,
    ) -> Result<impl AsyncRead + AsyncWrite + 'static, SerialManagerOpenPortError> {
        Ok(tokio::io::duplex(1024).0)
    }
}
