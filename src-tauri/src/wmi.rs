use std::{collections::HashMap, thread, time::Duration};

use anyhow::Context;
use serde::Deserialize;
use tauri::{AppHandle, Manager};
use wmi::{COMLibrary, FilterValue, WMIConnection};

#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_SerialPort")]
pub struct SerialPort {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "SystemName")]
    pub system_name: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename = "__InstanceDeletionEvent")]
#[serde(rename_all = "PascalCase")]
pub struct SerialDeletion {
    pub target_instance: SerialPort,
}

#[derive(Deserialize, Debug)]
#[serde(rename = "__InstanceCreationEvent")]
#[serde(rename_all = "PascalCase")]
pub struct SerialCreation {
    pub target_instance: SerialPort,
}

fn filters() -> anyhow::Result<HashMap<String, FilterValue>> {
    let mut filters = HashMap::<String, FilterValue>::new();
    filters.insert(
        "TargetInstance".to_owned(),
        FilterValue::is_a::<SerialPort>().context("Failed to create filter")?,
    );

    Ok(filters)
}

pub fn spawn_serial_events_watchers(app: AppHandle) -> anyhow::Result<()> {
    let app_handle_creation = app.app_handle().clone();
    let app_handle_deletion = app.app_handle().clone();

    thread::spawn(move || {
        tracing::debug!("Starting serial creation events watcher");

        let com_con = COMLibrary::new().context("Failed to create COM library")?;
        let wmi_con = WMIConnection::new(com_con).context("Failed to create WMI connection")?;
        let filters = filters()?;

        let creation_iter = wmi_con
            .filtered_notification::<SerialCreation>(&filters, Some(Duration::from_millis(300)))
            .context("Failed to create creation iterator")?;

        for creation_event in creation_iter {
            let Ok(creation_event) = creation_event else {
                break;
            };

            tracing::trace!(name=%creation_event.target_instance.name, "Serial creation event detected");

            if let Ok(ports) = crate::serial::available_ports() {
                let _ = app_handle_creation.emit_all("serial_ports_event", &ports);
            }
        }

        anyhow::Result::<()>::Ok(())
    });

    thread::spawn(move || {
        tracing::debug!("Starting serial deletion events watcher");

        let com_con = COMLibrary::new().context("Failed to create COM library")?;
        let wmi_con = WMIConnection::new(com_con).context("Failed to create WMI connection")?;
        let filters = filters()?;

        let deletion_iter = wmi_con
            .filtered_notification::<SerialDeletion>(&filters, Some(Duration::from_millis(300)))
            .context("Failed to create deletion iterator")?;

        for deletion_event in deletion_iter {
            let Ok(deletion_event) = deletion_event else {
                break;
            };

            tracing::trace!(name=%deletion_event.target_instance.name, "Serial deletion event detected");

            if let Ok(ports) = crate::serial::available_ports() {
                let _ = app_handle_deletion.emit_all("serial_ports_event", &ports);
            }
        }

        anyhow::Result::<()>::Ok(())
    });

    Ok(())
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
            FilterValue::is_a::<SerialPort>().expect("Failed to create filter"),
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
            FilterValue::is_a::<SerialPort>().expect("Failed to create filter"),
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
