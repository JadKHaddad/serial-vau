#[derive(Debug, thiserror::Error)]
pub enum WatcherPortEventError {
    #[error("Failed to create watcher port event: {0}")]
    Create(#[source] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum WatcherPortEventStreamError {
    #[error("Failed to create watcher port event stream: {0}")]
    Create(#[source] anyhow::Error),
}
