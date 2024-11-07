#[derive(Debug, thiserror::Error)]
pub enum SerialManagerAvailablePortsError {
    #[error("Failed to get available ports: {0}")]
    Get(#[source] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum SerialManagerOpenPortError {
    #[error("Failed to open port: {0}")]
    Open(#[source] anyhow::Error),
}
