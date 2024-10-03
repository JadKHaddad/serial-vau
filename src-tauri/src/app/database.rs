use partial::serial_port::SerialPortId;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, ConnectOptions, DatabaseConnection, EntityTrait,
    QueryFilter,
};

use crate::core::state::open_serial_port::CorePacket;

use super::model::managed_serial_port::AppOpenSerialPortOptions;

pub mod entity;
mod entity_impl;
mod partial;

#[derive(Debug, thiserror::Error)]
pub enum NewDatabaseError {
    #[error("Failed to connect to database: {0}")]
    Connect(#[from] sea_orm::error::DbErr),
}

#[derive(Debug)]
pub struct Database {
    conn: DatabaseConnection,
}

impl Database {
    pub async fn new(connection_string: &str) -> Result<Self, NewDatabaseError> {
        let connect_options = ConnectOptions::new(connection_string).to_owned();

        let conn = sea_orm::Database::connect(connect_options).await?;

        Ok(Self { conn })
    }

    /// When opening a serial port, get the `id` and maintain it to be able to insert [`AppOpenSerialPortOptions`] and [`CorePacket`].
    ///
    /// See [`Self::insert_open_serial_port_options`] and [`Self::insert_packet`].
    pub async fn get_serial_port_id_or_insert_returning_id(
        &self,
        name: &str,
    ) -> Result<i32, InsertSerialPortError> {
        let serial_port = entity::serial_port::Entity::find()
            .filter(entity::serial_port::Column::Name.eq(name))
            .into_partial_model::<SerialPortId>()
            .one(&self.conn)
            .await?;

        match serial_port {
            Some(serial_port) => Ok(serial_port.id),
            None => self.insert_serial_port_returning_id(name.to_string()).await,
        }
    }

    async fn insert_serial_port_returning_id(
        &self,
        name: String,
    ) -> Result<i32, InsertSerialPortError> {
        let serial_port = entity::serial_port::ActiveModel {
            name: ActiveValue::Set(name),
            ..Default::default()
        };

        let id = serial_port.insert(&self.conn).await?.id;

        Ok(id)
    }

    /// See [`Self::get_serial_port_id_or_insert_returning_id`].
    pub async fn insert_open_serial_port_options(
        &self,
        port_id: i32,
        options: AppOpenSerialPortOptions,
    ) -> Result<i32, InsertOpenSerialPortOptionsError> {
        let options = entity::open_options::ActiveModel::from((port_id, options));

        let id = options.insert(&self.conn).await?.id;

        Ok(id)
    }

    /// See [`Self::get_serial_port_id_or_insert_returning_id`].
    pub async fn insert_packet(
        &self,
        port_id: i32,
        tag: String,
        packet: CorePacket,
    ) -> Result<i32, InsertPacketError> {
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
    #[error("Failed to insert open serial port options: {0}")]
    Insert(#[from] sea_orm::error::DbErr),
}

#[derive(Debug, thiserror::Error)]
pub enum InsertPacketError {
    #[error("Failed to insert packet: {0}")]
    Insert(#[from] sea_orm::error::DbErr),
}
