use tokio::io::{AsyncRead, AsyncWrite};

use super::{
    error::{SerialManagerAvailablePortsError, SerialManagerOpenPortError},
    models::{SerialManagerOpenSerialPortOptions, SerialManagerPort},
};

pub trait SerialManagerService {
    fn available_ports(&self) -> Result<Vec<SerialManagerPort>, SerialManagerAvailablePortsError>;
    fn open_port<'a>(
        &self,
        name: impl Into<std::borrow::Cow<'a, str>>,
        options: SerialManagerOpenSerialPortOptions,
    ) -> Result<impl AsyncRead + AsyncWrite + 'static, SerialManagerOpenPortError>;
}
