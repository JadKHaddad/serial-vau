use std::io::Error as IOError;

use crate::{
    app::serial_state::codec::lines_codec::LinesCodecError,
    serial_manager::error::{SerialManagerAvailablePortsError, SerialManagerOpenPortError},
};

/// Error returned by [`StateInner::managed_serial_ports`](crate::core::state::StateInner::managed_serial_ports).
#[derive(Debug, thiserror::Error)]
pub enum CoreManagedSerialPortsError {
    #[error("Failed to get available ports: {0}")]
    AvailablePortsError(
        #[source]
        #[from]
        SerialManagerAvailablePortsError,
    ),
}

/// Error emitted by [`CoreSerialState::open_serial_port`](crate::core::state::CoreSerialState::open_serial_port) through the channel.
#[derive(Debug, thiserror::Error)]
pub enum CorePacketError {
    #[error("Incoming packet error: {0}")]
    Incoming(
        #[source]
        #[from]
        CoreIncomingPacketError,
    ),
    #[error("Outgoing packet error: {0}")]
    Outgoing(
        #[source]
        #[from]
        CoreOutgoingPacketError,
    ),
}

/// Internal part of [`CorePacketError`].
#[derive(Debug, thiserror::Error)]
pub enum CoreIncomingPacketError {
    #[error("An IO error occurred: {0}")]
    IO(
        #[source]
        #[from]
        IOError,
    ),
    #[error("Failed to decode packet: {0}")]
    Codec(
        #[source]
        #[from]
        LinesCodecError,
    ),
}

/// Internal part of [`CorePacketError`].
#[derive(Debug, thiserror::Error)]
pub enum CoreOutgoingPacketError {
    #[error("An IO error occurred: {0}")]
    IO(
        #[source]
        #[from]
        IOError,
    ),
}

/// Error returned by [`CoreSerialState::open_serial_port`](crate::core::state::CoreSerialState::open_serial_port).
#[derive(Debug, thiserror::Error)]
pub enum CoreOpenSerialPortError {
    #[error("Failed to get managed ports: {0}")]
    ManagedSerialPortsError(
        #[source]
        #[from]
        CoreManagedSerialPortsError,
    ),
    #[error("Port not found")]
    NotFound,
    #[error("Port already open")]
    AlreadyOpen,
    #[error("Failed to open port: {0}")]
    FailedToOpen(
        #[source]
        #[from]
        SerialManagerOpenPortError,
    ),
}
