#[derive(Debug, thiserror::Error)]
pub enum GetSerialPortError {
    #[error("Failed to get serial port: {0}")]
    Get(#[source] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum InsertSerialPortError {
    #[error("Failed to insert serial port: {0}")]
    Insert(#[source] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum GetOrInsertSerialPortError {
    #[error("Failed to get serial port: {0}")]
    Get(
        #[from]
        #[source]
        GetSerialPortError,
    ),
    #[error("Failed to insert serial port: {0}")]
    Insert(
        #[from]
        #[source]
        InsertSerialPortError,
    ),
}

#[derive(Debug, thiserror::Error)]
pub enum InsertOpenSerialPortOptionsError {
    #[error("Failed to insert open serial port options: {0}")]
    Insert(#[source] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum UpdateOpenSerialPortOptionsError {
    #[error("Failed to update open serial port options: {0}")]
    Update(#[source] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum UpdateOrInsertOpenSerialPortOptionsError {
    #[error("Failed to update open serial port options: {0}")]
    Update(
        #[from]
        #[source]
        UpdateOpenSerialPortOptionsError,
    ),
    #[error("Failed to insert open serial port options: {0}")]
    Insert(
        #[from]
        #[source]
        InsertOpenSerialPortOptionsError,
    ),
}

#[derive(Debug, thiserror::Error)]
pub enum InsertPacketError {
    #[error("Failed to insert packet: {0}")]
    Insert(#[source] anyhow::Error),
}
