use crate::watcher::{
    error::{WatcherPortEventError, WatcherPortEventStreamError},
    models::WatcherPortEvent,
    watcher_service::WatcherService,
};

#[derive(Debug)]
pub struct DummyWatcher;

impl DummyWatcher {
    pub fn new() -> Self {
        tracing::info!("Creating Dummy watcher");

        Self
    }
}

impl Default for DummyWatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl WatcherService for DummyWatcher {
    fn events_stream(
        &self,
    ) -> Result<
        impl futures::Stream<Item = Result<WatcherPortEvent, WatcherPortEventError>> + '_,
        WatcherPortEventStreamError,
    > {
        Ok(futures::stream::empty())
    }
}
