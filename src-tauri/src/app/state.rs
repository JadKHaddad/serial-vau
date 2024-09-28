use std::{collections::HashMap, ops::Deref, sync::Arc};

use error::{AddOrUpdateOpenSerialPortOptionsError, AddPacketError, GetOpenSerialPortOptionsError};

use crate::core::state::open_serial_port::{
    OpenSerialPortOptions as CoreOpenSerialPortOptions, Packet as CorePacket,
};

use super::model::{open_serial_port_options::OpenSerialPortOptions, packet::Packet};

pub mod error;

// TODO after implementing the database and adding the models make the models From/Into CoreModels like src-tauri/src/tauri_app/model/managed_serial_port.rs

/// Intended to save the packets and open options for serial ports.
#[derive(Debug, Default, Clone)]
pub struct State {
    inner: Arc<StateInner>,
}

impl Deref for State {
    type Target = StateInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[derive(Debug, Default)]
pub struct StateInner {}

impl StateInner {
    /// Add the `packet` to the internal buffer and flush it to the database eventually.
    pub async fn add_packet(&self, packet: &CorePacket) -> Result<(), AddPacketError> {
        // TODO: Implement this.

        Ok(())
    }

    /// Get the packets for the `port_name`.
    pub async fn get_packets(&self, port_name: &str) -> Result<Packet, AddPacketError> {
        // TODO: Implement this.

        Ok(Packet::default())
    }

    /// If the `open_options` already exists, update it. and save it to the database.
    /// If the `open_options` does not exist, add it and save it to the database.
    pub async fn add_or_update_open_serial_port_options(
        &self,
        port_name: &str,
        options: &CoreOpenSerialPortOptions,
    ) -> Result<(), AddOrUpdateOpenSerialPortOptionsError> {
        // TODO: Implement this.

        Ok(())
    }

    /// Get the open options for the `port_name`.
    pub async fn get_open_serial_port_options(
        &self,
        port_name: &str,
    ) -> Result<OpenSerialPortOptions, GetOpenSerialPortOptionsError> {
        // TODO: Implement this.

        Ok(OpenSerialPortOptions::default())
    }

    /// Get all the open options for all the open serial ports.
    ///
    /// Returns a map of the port name to the open options.
    pub async fn get_all_open_serial_port_options(
        &self,
    ) -> Result<HashMap<String, OpenSerialPortOptions>, GetOpenSerialPortOptionsError> {
        // TODO: Implement this.

        Ok(HashMap::new())
    }
}
