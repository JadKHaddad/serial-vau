use derive_more::From;
use error::{SerialManagerAvailablePortsError, SerialManagerOpenPortError};
use models::{SerialManagerOpenSerialPortOptions, SerialManagerPort};
use serial_manager_impl::{
    dummy_serial_manager::DummySerialManager, tokio_serial_manager::TokioSerialManager,
};
use serial_manager_service::SerialManagerService;
use tokio::io::{AsyncRead, AsyncWrite};

pub mod error;
pub mod models;
pub mod serial_manager_impl;
pub mod serial_manager_service;

#[derive(Debug, From)]
pub enum SerialManager {
    TokioSerialManager(TokioSerialManager),
    DummySerialManager(DummySerialManager),
}

impl SerialManagerService for SerialManager {
    fn available_ports(&self) -> Result<Vec<SerialManagerPort>, SerialManagerAvailablePortsError> {
        match self {
            Self::TokioSerialManager(manager) => manager.available_ports(),
            Self::DummySerialManager(manager) => manager.available_ports(),
        }
    }

    fn open_port<'a>(
        &self,
        name: impl Into<std::borrow::Cow<'a, str>>,
        options: SerialManagerOpenSerialPortOptions,
    ) -> Result<impl AsyncRead + AsyncWrite + 'static, SerialManagerOpenPortError> {
        #[auto_enums::enum_derive(tokio1::AsyncWrite, tokio1::AsyncRead)]
        enum Enum<A, B> {
            A(A),
            B(B),
        }

        match self {
            Self::TokioSerialManager(manager) => Ok(Enum::A(manager.open_port(name, options)?)),
            Self::DummySerialManager(manager) => Ok(Enum::B(manager.open_port(name, options)?)),
        }
    }
}
