use crate::{
    app::database::error::{
        InsertPacketError, InsertSerialPortError, UpdateOrInsertOpenSerialPortOptionsError,
    },
    core::state::error::{CoreManagedSerialPortsError, CoreOpenSerialPortError, CorePacketError},
};

#[derive(Debug, thiserror::Error)]
pub enum AppAddPacketError {}

#[derive(Debug, thiserror::Error)]
pub enum AppAddOrUpdateOpenSerialPortOptionsError {}

#[derive(Debug, thiserror::Error)]
pub enum AppGetOpenSerialPortOptionsError {}

#[derive(Debug, thiserror::Error)]
pub enum AppManagedSerialPortsError {
    #[error("Failed to get managed ports: {0}")]
    ManagedSerialPortsError(
        #[source]
        #[from]
        CoreManagedSerialPortsError,
    ),
    #[error("Failed to get open serial port options: {0}")]
    GetOpenSerialPortOptionsError(
        #[source]
        #[from]
        AppGetOpenSerialPortOptionsError,
    ),
}

#[derive(Debug, thiserror::Error)]
pub enum AppOpenSerialPortError {
    #[error("Failed to save serial port: {0}")]
    SertialPortId(
        #[source]
        #[from]
        InsertSerialPortError,
    ),
    #[error("Failed to save open serial port options: {0}")]
    SaveOpenOptions(
        #[source]
        #[from]
        UpdateOrInsertOpenSerialPortOptionsError,
    ),
    #[error("Failed to open serial port: {0}")]
    CoreOpenSerialPortError(
        #[source]
        #[from]
        CoreOpenSerialPortError,
    ),
}

#[derive(Debug, thiserror::Error)]
pub enum AppPacketError {
    #[error("Core packet error: {0}")]
    CorePacketError(
        #[source]
        #[from]
        CorePacketError,
    ),
    #[error("Failed to save packet: {0}")]
    SavePacketError(
        #[source]
        #[from]
        InsertPacketError,
    ),
}
