// TODO: the small esp32-c3 with micro usb is not detected by this watcher (using: https://github.com/esp-rs/esp-hal/blob/v0.20.0/examples/src/bin/embassy_serial.rs)

use std::{collections::HashMap, time::Duration};

use futures::{stream::select, Stream, StreamExt};
use serde::Deserialize;
use wmi::{COMLibrary, FilterValue, WMIConnection, WMIError};

use super::super::SerialPort;

#[derive(Debug)]
pub struct SerialPortEvent {
    pub event_type: SerialEventType,
    pub serial_port: SerialPort,
}

#[derive(Debug)]
pub enum SerialEventType {
    Creation,
    Deletion,
}

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

impl From<SerialDeletion> for SerialPort {
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

impl From<SerialCreation> for SerialPort {
    fn from(value: SerialCreation) -> Self {
        Self::new(value.target_instance.name)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum NewWatcherError {
    #[error("Failed to create library: {0}")]
    Lib(#[source] WMIError),
    #[error("Failed to create connection: {0}")]
    Con(#[source] WMIError),
}

#[derive(Debug, thiserror::Error)]
pub enum CreateFilterError {
    #[error("Failed to create filter value: {0}")]
    FilterValue(
        #[source]
        #[from]
        WMIError,
    ),
}

#[derive(Debug, thiserror::Error)]
pub enum CreateStreamError {
    #[error("Failed to create filter: {0}")]
    Filter(
        #[source]
        #[from]
        CreateFilterError,
    ),
    #[error("Failed to create notifications iter: {0}")]
    Notification(
        #[source]
        #[from]
        WMIError,
    ),
}

pub struct Watcher {
    wmi_con: WMIConnection,
}

impl Watcher {
    pub fn new() -> Result<Watcher, NewWatcherError> {
        let com_con = COMLibrary::new().map_err(NewWatcherError::Lib)?;
        let wmi_con = WMIConnection::new(com_con).map_err(NewWatcherError::Con)?;
        Ok(Self { wmi_con })
    }

    fn filters() -> Result<HashMap<String, FilterValue>, CreateFilterError> {
        let mut filters = HashMap::<String, FilterValue>::new();
        filters.insert(
            "TargetInstance".to_owned(),
            FilterValue::is_a::<Win32SerialPortEvent>()?,
        );

        Ok(filters)
    }

    fn creation_stream(
        &self,
    ) -> Result<impl Stream<Item = Result<SerialPortEvent, WMIError>> + '_, CreateStreamError> {
        let filters = Watcher::filters()?;

        let creation_stream = self
            .wmi_con
            .async_filtered_notification::<SerialCreation>(
                &filters,
                Some(Duration::from_millis(300)),
            )?
            .map(|event| {
                event.map(|event| SerialPortEvent {
                    event_type: SerialEventType::Creation,
                    serial_port: event.into(),
                })
            });

        Ok(creation_stream)
    }

    fn deletion_stream(
        &self,
    ) -> Result<impl Stream<Item = Result<SerialPortEvent, WMIError>> + '_, CreateStreamError> {
        let filters = Watcher::filters()?;

        let deletion_stream = self
            .wmi_con
            .async_filtered_notification::<SerialDeletion>(
                &filters,
                Some(Duration::from_millis(300)),
            )?
            .map(|event| {
                event.map(|event| SerialPortEvent {
                    event_type: SerialEventType::Deletion,
                    serial_port: event.into(),
                })
            });

        Ok(deletion_stream)
    }

    pub fn serial_port_events_stream(
        &self,
    ) -> Result<impl Stream<Item = Result<SerialPortEvent, WMIError>> + '_, CreateStreamError> {
        Ok(select(self.creation_stream()?, self.deletion_stream()?))
    }
}

#[cfg(test)]
mod test {
    use std::pin::pin;

    use futures::{stream::select, StreamExt};

    use super::*;

    #[tokio::test]
    #[ignore]
    // cargo test --package serial-vau --lib -- core::serial::watcher::test::watch --exact --show-output --ignored --nocapture
    async fn watch() {
        let pool = tokio_util::task::LocalPoolHandle::new(4);

        pool.spawn_pinned(|| {
            let com_con = COMLibrary::new().expect("Failed to create COM library");
            let wmi_con = WMIConnection::new(com_con).expect("Failed to create WMI connection");

            let mut filters = HashMap::<String, FilterValue>::new();

            filters.insert(
                "TargetInstance".to_owned(),
                FilterValue::is_a::<Win32SerialPortEvent>().expect("Failed to create filter"),
            );

            let deletion_stream = wmi_con
                .async_filtered_notification::<SerialDeletion>(
                    &filters,
                    Some(Duration::from_millis(300)),
                )
                .expect("Failed to create deletion stream")
                .filter_map(|event| async move { event.ok() })
                .map(|event| SerialPortEvent {
                    event_type: SerialEventType::Deletion,
                    serial_port: SerialPort::new(event.target_instance.name),
                });

            let mut filters = HashMap::<String, FilterValue>::new();

            filters.insert(
                "TargetInstance".to_owned(),
                FilterValue::is_a::<Win32SerialPortEvent>().expect("Failed to create filter"),
            );

            let creation_stream = wmi_con
                .async_filtered_notification::<SerialCreation>(
                    &filters,
                    Some(Duration::from_millis(300)),
                )
                .expect("Failed to create creation stream")
                .filter_map(|event| async move { event.ok() })
                .map(|event| SerialPortEvent {
                    event_type: SerialEventType::Creation,
                    serial_port: SerialPort::new(event.target_instance.name),
                });

            async move {
                let mut stream = pin!(select(deletion_stream, creation_stream));

                while let Some(event) = stream.next().await {
                    println!("{event:?}");
                }
            }
        })
        .await
        .unwrap();
    }
}
