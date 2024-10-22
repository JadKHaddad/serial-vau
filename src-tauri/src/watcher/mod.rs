use derive_more::From;
use error::{WatcherPortEventError, WatcherPortEventStreamError};
use futures::Stream;
use models::WatcherPortEvent;
use watcher_service::WatcherService;

pub mod error;
pub mod models;
pub mod watcher_impl;
pub mod watcher_service;

#[derive(Debug, From)]
pub enum Watcher {
    WatcherImpl(watcher_impl::watcher::WatcherImpl),
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
        enum Enum<A, B> {
            A(A),
            B(B),
        }

        match self {
            Self::WatcherImpl(watcher) => Ok(Enum::A(watcher.events_stream()?)),
            Self::DummyWatcher(watcher) => Ok(Enum::B(watcher.events_stream()?)),
        }
    }
}
