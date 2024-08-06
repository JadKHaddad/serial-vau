use std::io::Error as IOError;

use tokio_serial::Error as TokioSerialError;
use tokio_util::codec::LinesCodecError;

use crate::core::serial::AvailablePortsError;

#[derive(Debug, thiserror::Error)]
pub enum ManagedSerialPortsError {
    #[error("Failed to get available ports: {0}")]
    AvailablePortsError(
        #[source]
        #[from]
        AvailablePortsError,
    ),
}

#[derive(Debug, thiserror::Error)]
pub enum PacketError {
    #[error(transparent)]
    Incoming(#[from] IncomingPacketError),
    #[error(transparent)]
    Outgoing(#[from] OutgoingPacketError),
}

#[derive(Debug, thiserror::Error)]
pub enum IncomingPacketError {
    #[error("An IO error occurred: {0}")]
    IO(
        #[source]
        #[from]
        IOError,
    ),
    #[error("Failed to decode incoming packet: {0}")]
    Codec(
        #[source]
        #[from]
        LinesCodecError,
    ),
}

#[derive(Debug, thiserror::Error)]
pub enum OutgoingPacketError {
    #[error("An IO error occurred: {0}")]
    IO(
        #[source]
        #[from]
        IOError,
    ),
}

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
