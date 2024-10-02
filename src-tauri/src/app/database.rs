use std::collections::HashMap;

use sea_orm::{ActiveModelTrait, ActiveValue, ConnectOptions, DatabaseConnection, EntityTrait};
use tokio::sync::RwLock;

use crate::core::state::open_serial_port::CorePacket;

use super::model::managed_serial_port::AppOpenSerialPortOptions;

pub mod entity;
mod entity_impl;

#[derive(Debug, thiserror::Error)]
pub enum NewDatabaseError {
    #[error("Failed to connect to database: {0}")]
    Connect(#[from] sea_orm::error::DbErr),

    #[error("Failed to get all serial ports: {0}")]
    GetAllSerialPortsError(#[from] GetAllSerialPortsError),
}

#[derive(Debug)]
pub struct Database {
    conn: DatabaseConnection,
    serial_ports_cache: RwLock<HashMap<String, i32>>,
}

impl Database {
    pub async fn new(connection_string: &str) -> Result<Self, NewDatabaseError> {
        let connect_options = ConnectOptions::new(connection_string).to_owned();

        let conn = sea_orm::Database::connect(connect_options).await?;
        let serial_ports = Self::get_all_serial_ports(&conn).await?;

        let mut serial_ports_cache = HashMap::new();

        for serial_port in serial_ports {
            serial_ports_cache.insert(serial_port.name, serial_port.id);
        }

        let serial_ports_cache = RwLock::new(serial_ports_cache);

        Ok(Self {
            conn,
            serial_ports_cache,
        })
    }

    // Use projection if some fields are added to the serial port table
    async fn get_all_serial_ports(
        conn: &DatabaseConnection,
    ) -> Result<Vec<entity::serial_port::Model>, GetAllSerialPortsError> {
        let serial_ports = entity::serial_port::Entity::find().all(conn).await?;

        Ok(serial_ports)
    }

    async fn get_serial_port_id_from_cache_or_insert_and_update_cache(
        &self,
        name: &str,
    ) -> Result<i32, InsertSerialPortError> {
        let id_opt = self.serial_ports_cache.read().await.get(name).cloned();

        match id_opt {
            Some(id) => Ok(id),
            None => {
                let id = self.insert_serial_port(name.to_string()).await?;

                self.serial_ports_cache
                    .write()
                    .await
                    .insert(name.to_string(), id);

                Ok(id)
            }
        }
    }

    async fn insert_serial_port(&self, name: String) -> Result<i32, InsertSerialPortError> {
        let serial_port = entity::serial_port::ActiveModel {
            name: ActiveValue::Set(name),
            ..Default::default()
        };

        let id = serial_port.insert(&self.conn).await?.id;

        Ok(id)
    }

    pub async fn insert_open_serial_port_options(
        &self,
        port_name: &str,
        options: AppOpenSerialPortOptions,
    ) -> Result<i32, InsertOpenSerialPortOptionsError> {
        let port_id = self
            .get_serial_port_id_from_cache_or_insert_and_update_cache(port_name)
            .await?;

        let options = entity::open_options::ActiveModel::from((port_id, options));

        let id = options.insert(&self.conn).await?.id;

        Ok(id)
    }

    pub async fn insert_packet(
        &self,
        port_name: &str,
        tag: String,
        packet: CorePacket,
    ) -> Result<i32, InsertPacketError> {
        let port_id = self
            .get_serial_port_id_from_cache_or_insert_and_update_cache(port_name)
            .await?;

        let packet = entity::packet::ActiveModel::from((port_id, tag, packet));

        let id = packet.insert(&self.conn).await?.id;

        Ok(id)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GetAllSerialPortsError {
    #[error("Failed to get all serial ports: {0}")]
    Get(#[from] sea_orm::error::DbErr),
}

#[derive(Debug, thiserror::Error)]
pub enum InsertSerialPortError {
    #[error("Failed to insert serial port: {0}")]
    Insert(#[from] sea_orm::error::DbErr),
}

#[derive(Debug, thiserror::Error)]
pub enum InsertOpenSerialPortOptionsError {
    #[error("Failed to get serial port id: {0}")]
    SerialPortId(#[from] InsertSerialPortError),

    #[error("Failed to insert open serial port options: {0}")]
    Insert(#[from] sea_orm::error::DbErr),
}

#[derive(Debug, thiserror::Error)]
pub enum InsertPacketError {
    #[error("Failed to get serial port id: {0}")]
    SerialPortId(#[from] InsertSerialPortError),

    #[error("Failed to insert packet: {0}")]
    Insert(#[from] sea_orm::error::DbErr),
}
