use futures::Stream;

use super::{
    error::{WatcherPortEventError, WatcherPortEventStreamError},
    models::WatcherPortEvent,
};

pub trait WatcherService {
    fn events_stream(
        &self,
    ) -> Result<
        impl Stream<Item = Result<WatcherPortEvent, WatcherPortEventError>> + '_,
        WatcherPortEventStreamError,
    >;
}
