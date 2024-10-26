use std::collections::HashMap;

use error::{
    AppAddPacketError, AppGetOpenSerialPortOptionsError, AppManagedSerialPortsError,
    AppOpenSerialPortError, AppPacketError,
};
use tokio::sync::mpsc::UnboundedReceiver as MPSCUnboundedReceiver;

use crate::serial_manager::SerialManager;

use super::{
    database::{Database, NewDatabaseError},
    model::managed_serial_port::{AppManagedSerialPort, AppOpenSerialPortOptions},
    serial_state::{model::CorePacket, CoreSerialState},
};

pub mod error;

#[derive(Debug, thiserror::Error)]
pub enum NewAppStateError {
    #[error("Failed to create database: {0}")]
    Database(#[from] NewDatabaseError),
}

/// Intended to save the packets and open options for serial ports.
#[derive(Debug, Clone)]
pub struct AppState {
    serial_state: CoreSerialState,
    db: Database,
}

impl AppState {
    pub async fn new(
        connection_string: &str,
        serial_manager: SerialManager,
    ) -> Result<Self, NewAppStateError> {
        let serial_state = CoreSerialState::new(serial_manager);

        // TODO: run the migrations!
        // We might need to create the sqlite database file first if (sqlite connection) and it doesn't exist.
        let db = Database::new(connection_string).await?;

        Ok(Self { serial_state, db })
    }

    pub fn serial_state(&self) -> &CoreSerialState {
        &self.serial_state
    }

    /// Get the packets for the `port_name`.
    pub async fn get_packets(&self, port_name: &str) -> Result<CorePacket, AppAddPacketError> {
        // TODO: Implement this.

        Ok(CorePacket::default())
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

    pub async fn open_serial_port(
        &self,
        name: &str,
        options: AppOpenSerialPortOptions,
    ) -> Result<MPSCUnboundedReceiver<Result<CorePacket, AppPacketError>>, AppOpenSerialPortError>
    {
        tracing::debug!(?options, "Opening serial port");

        // get the serial port id
        let port_id = self
            .db
            // TODO: maybe some chaching for the serial port id
            .get_serial_port_id_or_insert_returning_id(name)
            .await?;

        // save the options
        let _ = self
            .db
            .update_or_insert_serial_port_options_returning_id(port_id, options.clone())
            .await?;

        let tag = options.tag;
        let mut core_rx = self
            .serial_state()
            .open_serial_port(name, options.core_options)
            .await?;

        let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<Result<CorePacket, AppPacketError>>();

        let name = name.to_string();
        let db = self.db.clone();
        tokio::spawn(async move {
            tracing::debug!(name=%name, "Read events task started");

            while let Some(packet) = core_rx.recv().await {
                match packet {
                    Ok(packet) => {
                        // TODO: maybe some buffering
                        match db
                            .insert_packet_returning_id(port_id, tag.clone(), packet.clone())
                            .await
                        {
                            Ok(id) => {
                                tracing::debug!(id, from=%name, "Packet saved");
                            }
                            Err(err) => {
                                tracing::error!(%err, from=%name, "Error saving packet");

                                let _ = tx.send(Err(AppPacketError::SavePacketError(err)));
                            }
                        }

                        let _ = tx.send(Ok(packet));
                    }
                    Err(err) => {
                        let _ = tx.send(Err(AppPacketError::CorePacketError(err)));
                    }
                }
            }

            tracing::debug!(name=%name, "Read events task terminated");
        });

        Ok(rx)
    }
}
