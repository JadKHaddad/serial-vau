// TODO: the small esp32-c3 with micro usb is not detected by this watcher (using: https://github.com/esp-rs/esp-hal/blob/v0.20.0/examples/src/bin/embassy_serial.rs)

use std::{collections::HashMap, time::Duration};

use futures::{stream::select, Stream, StreamExt};
use serde::Deserialize;
use wmi::{COMLibrary, FilterValue, WMIConnection, WMIError};

use crate::watcher::{
    error::{WatcherPortEventError, WatcherPortEventStreamError},
    models::{WatcherEventType, WatcherPort, WatcherPortEvent},
    watcher_service::WatcherService,
};

#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_SerialPort")]
#[serde(rename_all = "PascalCase")]
pub struct Win32SerialPortEvent {
    name: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename = "__InstanceDeletionEvent")]
#[serde(rename_all = "PascalCase")]
struct SerialDeletion {
    target_instance: Win32SerialPortEvent,
}

impl From<SerialDeletion> for WatcherPort {
    fn from(value: SerialDeletion) -> Self {
        Self::new(value.target_instance.name)
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename = "__InstanceCreationEvent")]
#[serde(rename_all = "PascalCase")]
struct SerialCreation {
    target_instance: Win32SerialPortEvent,
}

impl From<SerialCreation> for WatcherPort {
    fn from(value: SerialCreation) -> Self {
        Self::new(value.target_instance.name)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum WMIWatcherNewError {
    #[error("Failed to create library: {0}")]
    Lib(#[source] WMIError),
    #[error("Failed to create connection: {0}")]
    Con(#[source] WMIError),
}

#[derive(Debug, thiserror::Error)]
pub enum WMIWatcherCreateFilterError {
    #[error("Failed to create filter value: {0}")]
    FilterValue(
        #[source]
        #[from]
        WMIError,
    ),
}

#[derive(Debug, thiserror::Error)]
pub enum WMIWatcherCreateStreamError {
    #[error("Failed to create filter: {0}")]
    Filter(
        #[source]
        #[from]
        WMIWatcherCreateFilterError,
    ),
    #[error("Failed to create notifications iter: {0}")]
    Notification(
        #[source]
        #[from]
        WMIError,
    ),
}

#[derive(Debug)]
pub struct WMIWatcher {
    wmi_con: WMIConnection,
}

impl WMIWatcher {
    pub fn new() -> Result<WMIWatcher, WMIWatcherNewError> {
        tracing::info!("Creating WMI watcher");

        let com_con = COMLibrary::new().map_err(WMIWatcherNewError::Lib)?;
        let wmi_con = WMIConnection::new(com_con).map_err(WMIWatcherNewError::Con)?;
        Ok(Self { wmi_con })
    }

    fn filters() -> Result<HashMap<String, FilterValue>, WMIWatcherCreateFilterError> {
        let mut filters = HashMap::<String, FilterValue>::new();
        filters.insert(
            "TargetInstance".to_owned(),
            FilterValue::is_a::<Win32SerialPortEvent>()?,
        );

        Ok(filters)
    }

    fn creation_stream(
        &self,
    ) -> Result<
        impl Stream<Item = Result<WatcherPortEvent, WatcherPortEventError>> + '_,
        WatcherPortEventStreamError,
    > {
        let filters =
            WMIWatcher::filters().map_err(|err| WatcherPortEventStreamError::Create(err.into()))?;

        let creation_stream = self
            .wmi_con
            .async_filtered_notification::<SerialCreation>(
                &filters,
                Some(Duration::from_millis(300)),
            )
            .map_err(|err| WatcherPortEventStreamError::Create(err.into()))?
            .map(|event| {
                event
                    .map(|event| WatcherPortEvent {
                        event_type: WatcherEventType::Creation,
                        serial_port: event.into(),
                    })
                    .map_err(|err| WatcherPortEventError::Create(err.into()))
            });

        Ok(creation_stream)
    }

    fn deletion_stream(
        &self,
    ) -> Result<
        impl Stream<Item = Result<WatcherPortEvent, WatcherPortEventError>> + '_,
        WatcherPortEventStreamError,
    > {
        let filters =
            WMIWatcher::filters().map_err(|err| WatcherPortEventStreamError::Create(err.into()))?;

        let deletion_stream = self
            .wmi_con
            .async_filtered_notification::<SerialDeletion>(
                &filters,
                Some(Duration::from_millis(300)),
            )
            .map_err(|err| WatcherPortEventStreamError::Create(err.into()))?
            .map(|event| {
                event
                    .map(|event| WatcherPortEvent {
                        event_type: WatcherEventType::Deletion,
                        serial_port: event.into(),
                    })
                    .map_err(|err| WatcherPortEventError::Create(err.into()))
            });

        Ok(deletion_stream)
    }
}

impl WatcherService for WMIWatcher {
    fn events_stream(
        &self,
    ) -> Result<
        impl Stream<Item = Result<WatcherPortEvent, WatcherPortEventError>> + '_,
        WatcherPortEventStreamError,
    > {
        Ok(select(self.creation_stream()?, self.deletion_stream()?))
    }
}
