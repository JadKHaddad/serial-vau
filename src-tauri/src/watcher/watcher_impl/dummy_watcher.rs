use crate::watcher::{
    error::{WatcherPortEventError, WatcherPortEventStreamError},
    models::WatcherPortEvent,
    watcher_service::WatcherService,
};

#[derive(Debug, Default)]
pub struct DummyWatcher;

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
