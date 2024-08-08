use std::io::Error as IOError;

use tokio_serial::Error as TokioSerialError;
use tokio_util::codec::LinesCodecError;

use crate::core::serial::AvailablePortsError;

/// Error returned by [`AppStateInner::managed_serial_ports`](crate::core::state::AppStateInner::managed_serial_ports).
#[derive(Debug, thiserror::Error)]
pub enum ManagedSerialPortsError {
    #[error("Failed to get available ports: {0}")]
    AvailablePortsError(
        #[source]
        #[from]
        AvailablePortsError,
    ),
}

/// Error emitted by [`AppState::open_serial_port`](crate::core::state::AppState::open_serial_port) through the channel.
#[derive(Debug, thiserror::Error)]
pub enum PacketError {
    #[error("Incoming packet error: {0}")]
    Incoming(
        #[source]
        #[from]
        IncomingPacketError,
    ),
    #[error("Outgoing packet error: {0}")]
    Outgoing(
        #[source]
        #[from]
        OutgoingPacketError,
    ),
}

/// Internal part of [`PacketError`].
#[derive(Debug, thiserror::Error)]
pub enum IncomingPacketError {
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
pub enum OutgoingPacketError {
    #[error("An IO error occurred: {0}")]
    IO(
        #[source]
        #[from]
        IOError,
    ),
}

/// Error returned by [`AppState::open_serial_port`](crate::core::state::AppState::open_serial_port).
#[derive(Debug, thiserror::Error)]
pub enum OpenSerialPortError {
    #[error("Failed to get managed ports: {0}")]
    ManagedSerialPortsError(
        #[source]
        #[from]
        ManagedSerialPortsError,
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
