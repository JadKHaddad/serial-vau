use sea_orm::{ActiveModelTrait, ActiveValue, ConnectOptions, DatabaseConnection, EntityTrait};

use crate::core::state::open_serial_port::CorePacket;

use super::model::managed_serial_port::AppOpenSerialPortOptions;

pub mod entity;
mod entity_impl;

#[derive(Debug, thiserror::Error)]
pub enum NewDatabaseError {
    #[error("Failed to connect to database: {0}")]
    Connect(#[from] sea_orm::error::DbErr),
}

#[derive(Clone)]
pub struct Database {
    db: DatabaseConnection,
}

impl Database {
    pub async fn new(connection_string: &str) -> Result<Self, NewDatabaseError> {
        // TODO: add serial port ids cache to get the id from the name
        // populate the cache on startup
        // learn the cache on insert if not exists
        let connect_options = ConnectOptions::new(connection_string).to_owned();

        let db = sea_orm::Database::connect(connect_options).await?;

        Ok(Self { db })
    }

    // Use projection if some fields are added to the serial port table
    async fn get_all_serial_ports(
        &self,
    ) -> Result<Vec<entity::serial_port::Model>, GetAllSerialPortsError> {
        let serial_ports = entity::serial_port::Entity::find().all(&self.db).await?;

        Ok(serial_ports)
    }

    pub async fn insert_serial_port(&self, name: String) -> Result<i32, InsertSerialPortError> {
        let serial_port = entity::serial_port::ActiveModel {
            name: ActiveValue::Set(name),
            ..Default::default()
        };

        let id = serial_port.insert(&self.db).await?.id;

        Ok(id)
    }

    pub async fn insert_open_serial_port_options(
        &self,
        port_id: i32,
        options: AppOpenSerialPortOptions,
    ) -> Result<i32, InsertOpenSerialPortOptionsError> {
        let options = entity::open_options::ActiveModel::from((port_id, options));

        let id = options.insert(&self.db).await?.id;

        Ok(id)
    }

    pub async fn insert_packet(
        &self,
        port_id: i32,
        tag: String,
        packet: CorePacket,
    ) -> Result<i32, InsertPacketError> {
        let packet = entity::packet::ActiveModel::from((port_id, tag, packet));

        let id = packet.insert(&self.db).await?.id;

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
    #[error("Failed to insert open serial port options: {0}")]
    Insert(#[from] sea_orm::error::DbErr),
}

#[derive(Debug, thiserror::Error)]
pub enum InsertPacketError {
    #[error("Failed to insert packet: {0}")]
    Insert(#[from] sea_orm::error::DbErr),
}
