// TODO: the small esp32-c3 with micro usb is not detected by this watcher (using: https://github.com/esp-rs/esp-hal/blob/v0.20.0/examples/src/bin/embassy_serial.rs)
use std::{collections::HashMap, time::Duration};

use serde::Deserialize;
use wmi::{COMLibrary, FilterValue, WMIConnection, WMIError};

use super::SerialPort;

#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_SerialPort")]
#[serde(rename_all = "PascalCase")]
pub struct SerialPortEvent {
    name: String,
}

impl From<SerialPortEvent> for SerialPort {
    fn from(value: SerialPortEvent) -> Self {
        Self::new(value.name)
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename = "__InstanceDeletionEvent")]
#[serde(rename_all = "PascalCase")]
struct SerialDeletion {
    target_instance: SerialPortEvent,
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
    target_instance: SerialPortEvent,
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
pub enum CreateIterError {
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
            FilterValue::is_a::<SerialPortEvent>()?,
        );

        Ok(filters)
    }

    pub fn creation_iter(
        &self,
    ) -> Result<impl Iterator<Item = Result<SerialPort, WMIError>> + '_, CreateIterError> {
        let filters = Watcher::filters()?;

        let creation_iter = self
            .wmi_con
            .filtered_notification::<SerialCreation>(&filters, Some(Duration::from_millis(300)))?
            .map(|item| item.map(Into::into));

        Ok(creation_iter)
    }

    pub fn deletion_iter(
        &self,
    ) -> Result<impl Iterator<Item = Result<SerialPort, WMIError>> + '_, CreateIterError> {
        let filters = Watcher::filters()?;

        let deletion_iter = self
            .wmi_con
            .filtered_notification::<SerialDeletion>(&filters, Some(Duration::from_millis(300)))?
            .map(|item| item.map(Into::into));

        Ok(deletion_iter)
    }
}

#[cfg(test)]
mod test {
    use std::pin::pin;

    use futures::{stream::select, StreamExt};

    use super::*;

    #[derive(Debug)]
    struct SerialEvent {
        event_type: SerialEventType,
        port_name: String,
    }

    #[derive(Debug)]
    enum SerialEventType {
        Creation,
        Deletion,
    }

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
                FilterValue::is_a::<SerialPortEvent>().expect("Failed to create filter"),
            );

            let deletion_stream = wmi_con
                .async_filtered_notification::<SerialDeletion>(
                    &filters,
                    Some(Duration::from_millis(300)),
                )
                .expect("Failed to create deletion stream")
                .filter_map(|event| async move { event.ok() })
                .map(|event| SerialEvent {
                    event_type: SerialEventType::Deletion,
                    port_name: event.target_instance.name,
                });

            let mut filters = HashMap::<String, FilterValue>::new();

            filters.insert(
                "TargetInstance".to_owned(),
                FilterValue::is_a::<SerialPortEvent>().expect("Failed to create filter"),
            );

            let creation_stream = wmi_con
                .async_filtered_notification::<SerialCreation>(
                    &filters,
                    Some(Duration::from_millis(300)),
                )
                .expect("Failed to create creation stream")
                .filter_map(|event| async move { event.ok() })
                .map(|event| SerialEvent {
                    event_type: SerialEventType::Creation,
                    port_name: event.target_instance.name,
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
