use std::{collections::HashMap, time::Duration};

use serde::Deserialize;
use wmi::{COMLibrary, FilterValue, WMIConnection, WMIError};

use crate::serial::SerialPort;

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

    pub fn creation_iter<'a>(
        &'a self,
    ) -> Result<impl Iterator<Item = Result<SerialPort, WMIError>> + 'a, CreateIterError> {
        let filters = Watcher::filters()?;

        let creation_iter = self
            .wmi_con
            .filtered_notification::<SerialCreation>(&filters, Some(Duration::from_millis(300)))?
            .map(|item| item.map(Into::into));

        Ok(creation_iter)
    }

    pub fn deletion_iter<'a>(
        &'a self,
    ) -> Result<impl Iterator<Item = Result<SerialPort, WMIError>> + 'a, CreateIterError> {
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
    use futures::StreamExt;

    use super::*;

    #[tokio::test]
    #[ignore]
    // cargo test --package serial-vau --lib -- serial::watcher::test::watch --exact --show-output --ignored --nocapture
    async fn watch() {
        let com_con = COMLibrary::new().expect("Failed to create COM library");
        let wmi_con = WMIConnection::new(com_con).expect("Failed to create WMI connection");

        let mut filters = HashMap::<String, FilterValue>::new();

        filters.insert(
            "TargetInstance".to_owned(),
            FilterValue::is_a::<SerialPortEvent>().expect("Failed to create filter"),
        );

        let mut deletion_stream = wmi_con
            .async_filtered_notification::<SerialDeletion>(
                &filters,
                Some(Duration::from_millis(300)),
            )
            .expect("Failed to create deletion stream");

        let mut filters = HashMap::<String, FilterValue>::new();

        filters.insert(
            "TargetInstance".to_owned(),
            FilterValue::is_a::<SerialPortEvent>().expect("Failed to create filter"),
        );

        let mut creation_stream = wmi_con
            .async_filtered_notification::<SerialCreation>(
                &filters,
                Some(Duration::from_millis(300)),
            )
            .expect("Failed to create creation stream");

        loop {
            tokio::select! {
                deletion_event =  deletion_stream.next() => {
                    let Some(Ok(deletion_event)) = deletion_event else {
                        break;
                    };

                    println!("{deletion_event:?}");
                },
                creation_event =  creation_stream.next() => {
                    let Some(Ok(creation_event)) = creation_event else {
                        break;
                    };

                    println!("{creation_event:?}");
                },
            }
        }
    }
}
