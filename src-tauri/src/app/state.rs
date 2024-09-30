use std::collections::HashMap;

use error::{
    AppAddOrUpdateOpenSerialPortOptionsError, AppAddPacketError, AppGetOpenSerialPortOptionsError,
};

use crate::core::state::{
    error::CoreManagedSerialPortsError,
    open_serial_port::{CoreOpenSerialPortOptions, CorePacket},
    CoreSerialState,
};

use super::model::managed_serial_port::{AppManagedSerialPort, AppOpenSerialPortOptions};

pub mod error;

// TODO after implementing the database and adding the models make the models From/Into CoreModels like src-tauri/src/tauri_app/model/managed_serial_port.rs

/// Intended to save the packets and open options for serial ports.
#[derive(Debug, Default, Clone)]
pub struct AppState {
    serial_state: CoreSerialState,
}

impl AppState {
    pub fn new(serial_state: CoreSerialState) -> Self {
        Self { serial_state }
    }

    pub fn serial_state(&self) -> &CoreSerialState {
        &self.serial_state
    }

    /// Add the `packet` to the internal buffer and flush it to the database eventually.
    pub async fn add_packet(&self, packet: &CorePacket) -> Result<(), AppAddPacketError> {
        // TODO: Implement this.

        Ok(())
    }

    /// Get the packets for the `port_name`.
    pub async fn get_packets(&self, port_name: &str) -> Result<CorePacket, AppAddPacketError> {
        // TODO: Implement this.

        Ok(CorePacket::default())
    }

    /// If the `open_options` already exists, update it. and save it to the database.
    /// If the `open_options` does not exist, add it and save it to the database.
    pub async fn add_or_update_open_serial_port_options(
        &self,
        port_name: &str,
        options: &AppOpenSerialPortOptions,
    ) -> Result<(), AppAddOrUpdateOpenSerialPortOptionsError> {
        // TODO: Implement this.

        Ok(())
    }

    /// Get the open options for the `port_name`.
    pub async fn get_open_serial_port_options(
        &self,
        port_name: &str,
    ) -> Result<AppOpenSerialPortOptions, AppGetOpenSerialPortOptionsError> {
        // TODO: Implement this.

        Ok(AppOpenSerialPortOptions::default())
    }

    /// Get all the open options for all the open serial ports.
    ///
    /// Returns a map of the port name to the open options.
    pub async fn get_all_open_serial_port_options(
        &self,
    ) -> Result<HashMap<String, AppOpenSerialPortOptions>, AppGetOpenSerialPortOptionsError> {
        // TODO: Implement this.

        Ok(HashMap::new())
    }

    pub async fn get_managed_serial_ports(
        &self,
    ) -> Result<Vec<AppManagedSerialPort>, AppManagedSerialPortsError> {
        let managed_serial_ports = self.serial_state().managed_serial_ports().await?;
        let mut open_serial_port_options = self.get_all_open_serial_port_options().await?;

        let managed_serial_ports = managed_serial_ports
            .into_iter()
            .map(|port| {
                let last_used_open_options = open_serial_port_options
                    .remove(&port.name)
                    .unwrap_or_default();

                AppManagedSerialPort {
                    managed_serial_port: port,
                    last_used_open_options,
                }
            })
            .collect();

        Ok(managed_serial_ports)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AppManagedSerialPortsError {
    #[error("Failed to get managed ports: {0}")]
    ManagedSerialPortsError(
        #[source]
        #[from]
        CoreManagedSerialPortsError,
    ),
    #[error("Failed to get open serial port options: {0}")]
    GetOpenSerialPortOptionsError(
        #[source]
        #[from]
        AppGetOpenSerialPortOptionsError,
    ),
}
