use error::{WatcherPortEventError, WatcherPortEventStreamError};
use futures::Stream;
use models::WatcherPortEvent;
use watcher_service::WatcherService;

pub mod error;
pub mod models;
pub mod watcher_impl;
pub mod watcher_service;

#[derive(Debug)]
pub enum Watcher {
    #[cfg(windows)]
    WmiWatcher(watcher_impl::wmi_watcher::WMIWatcher),
    DummyWatcher(watcher_impl::dummy_watcher::DummyWatcher),
}

impl WatcherService for Watcher {
    fn events_stream(
        &self,
    ) -> Result<
        impl Stream<Item = Result<WatcherPortEvent, WatcherPortEventError>> + '_,
        WatcherPortEventStreamError,
    > {
        #[auto_enums::enum_derive(futures03::Stream)]
        enum EventStreamDispatch<A, B> {
            A(A),
            B(B),
        }

        match self {
            #[cfg(windows)]
            Self::WmiWatcher(watcher) => Ok(EventStreamDispatch::A(watcher.events_stream()?)),
            Self::DummyWatcher(watcher) => Ok(EventStreamDispatch::B(watcher.events_stream()?)),
        }
    }
}
