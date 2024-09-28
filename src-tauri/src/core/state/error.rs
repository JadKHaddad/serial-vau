use std::io::Error as IOError;

use tokio_serial::Error as TokioSerialError;

use crate::core::{codec::lines_codec::LinesCodecError, serial::AvailablePortsError};

/// Error returned by [`StateInner::managed_serial_ports`](crate::core::state::StateInner::managed_serial_ports).
#[derive(Debug, thiserror::Error)]
pub enum CoreManagedSerialPortsError {
    #[error("Failed to get available ports: {0}")]
    AvailablePortsError(
        #[source]
        #[from]
        AvailablePortsError,
    ),
}

/// Error emitted by [`State::open_serial_port`](crate::core::state::State::open_serial_port) through the channel.
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

/// Internal part of [`PacketError`].
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

/// Internal part of [`PacketError`].
#[derive(Debug, thiserror::Error)]
pub enum CoreOutgoingPacketError {
    #[error("An IO error occurred: {0}")]
    IO(
        #[source]
        #[from]
        IOError,
    ),
}

/// Error returned by [`State::open_serial_port`](crate::core::state::State::open_serial_port).
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
        TokioSerialError,
    ),
}
