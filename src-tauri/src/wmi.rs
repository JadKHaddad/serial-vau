use std::{collections::HashMap, time::Duration};

use anyhow::Context;
use serde::Deserialize;
use wmi::{COMLibrary, FilterValue, WMIConnection, WMIError};

use crate::serial::SerialPort;

#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_SerialPort")]
pub struct SerialPortEvent {
    #[serde(rename = "Name")]
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

pub struct Con {
    wmi_con: WMIConnection,
}

impl Con {
    pub fn new() -> anyhow::Result<Con> {
        let com_con = COMLibrary::new().context("Failed to create COM library")?;
        let wmi_con = WMIConnection::new(com_con).context("Failed to create WMI connection")?;
        Ok(Self { wmi_con })
    }

    fn filters() -> anyhow::Result<HashMap<String, FilterValue>> {
        let mut filters = HashMap::<String, FilterValue>::new();
        filters.insert(
            "TargetInstance".to_owned(),
            FilterValue::is_a::<SerialPortEvent>().context("Failed to create filter")?,
        );

        Ok(filters)
    }

    pub fn creation_iter<'a>(
        &'a self,
    ) -> anyhow::Result<impl Iterator<Item = Result<SerialPort, WMIError>> + 'a> {
        let filters = Con::filters()?;

        let creation_iter = self
            .wmi_con
            .filtered_notification::<SerialCreation>(&filters, Some(Duration::from_millis(300)))
            .context("Failed to create creation iterator")?
            .map(|item| item.map(Into::into));

        Ok(creation_iter)
    }

    pub fn deletion_iter<'a>(
        &'a self,
    ) -> anyhow::Result<impl Iterator<Item = Result<SerialPort, WMIError>> + 'a> {
        let filters = Con::filters()?;

        let deletion_iter = self
            .wmi_con
            .filtered_notification::<SerialDeletion>(&filters, Some(Duration::from_millis(300)))
            .context("Failed to create deletion iterator")?
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
    // cargo test --package serial-vau --lib -- wmi::test::watch --exact --show-output --ignored --nocapture
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
