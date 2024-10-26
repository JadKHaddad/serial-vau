use futures::Stream;

use super::{
    error::{WatcherPortEventError, WatcherPortEventStreamError},
    model::WatcherPortEvent,
};

pub trait WatcherService {
    fn events_stream(
        &self,
    ) -> Result<
        impl Stream<Item = Result<WatcherPortEvent, WatcherPortEventError>> + '_,
        WatcherPortEventStreamError,
    >;
}
