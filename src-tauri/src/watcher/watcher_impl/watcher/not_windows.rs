use std::convert::Infallible;

use crate::watcher::{
    error::{WatcherPortEventError, WatcherPortEventStreamError},
    models::WatcherPortEvent,
    watcher_service::WatcherService,
};

#[derive(Debug, Default)]
pub struct NotWindowsWatcher {
    _private: (),
}

impl NotWindowsWatcher {
    pub fn new() -> Result<Self, Infallible> {
        tracing::info!("Creating not windows Watcher");

        Ok(Self { _private: () })
    }
}

impl WatcherService for NotWindowsWatcher {
    fn events_stream(
        &self,
    ) -> Result<
        impl futures::Stream<Item = Result<WatcherPortEvent, WatcherPortEventError>> + '_,
        WatcherPortEventStreamError,
    > {
        Ok(futures::stream::empty())
    }
}
