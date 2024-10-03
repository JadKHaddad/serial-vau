#[derive(Debug, thiserror::Error)]
pub enum GetAllSerialPortsError {
    #[error("Failed to get all serial ports: {0}")]
    Get(#[from] sea_orm::error::DbErr),
}

#[derive(Debug, thiserror::Error)]
pub enum InsertSerialPortError {
    #[error("Failed to insert serial port: {0}")]
    Insert(#[from] sea_orm::error::DbErr),
}

#[derive(Debug, thiserror::Error)]
pub enum InsertOpenSerialPortOptionsError {
    #[error("Failed to insert open serial port options: {0}")]
    Insert(#[from] sea_orm::error::DbErr),
}

#[derive(Debug, thiserror::Error)]
pub enum UpdateOrInsertOpenSerialPortOptionsError {
    #[error("Failed to get open serial port options: {0}")]
    Get(sea_orm::error::DbErr),
    #[error("Failed to update open serial port options: {0}")]
    Update(sea_orm::error::DbErr),
    #[error("Failed to insert open serial port options: {0}")]
    Insert(InsertOpenSerialPortOptionsError),
}

#[derive(Debug, thiserror::Error)]
pub enum InsertPacketError {
    #[error("Failed to insert packet: {0}")]
    Insert(#[from] sea_orm::error::DbErr),
}
