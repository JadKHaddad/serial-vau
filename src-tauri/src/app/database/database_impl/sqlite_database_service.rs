use partial::serial_port::SerialPortId;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, ConnectOptions, DatabaseConnection, EntityTrait,
    QueryFilter,
};

use crate::app::{
    database::{
        database_service::DatabaseService,
        error::{
            GetSerialPortError, InsertPacketError, InsertSerialPortError,
            UpdateOrInsertOpenSerialPortOptionsError,
        },
        model::UpdateOrInsert,
    },
    model::managed_serial_port::AppOpenSerialPortOptions,
    serial_state::model::CorePacket,
};

pub mod entity;
mod entity_impl;
mod partial;

#[derive(Debug, thiserror::Error)]
pub enum NewDatabaseError {
    #[error("Failed to connect to database: {0}")]
    Connect(#[from] sea_orm::error::DbErr),
}

#[derive(Debug, Clone)]
pub struct SqliteDatabase {
    conn: DatabaseConnection,
}

impl SqliteDatabase {
    pub async fn new(connection_string: &str) -> Result<Self, NewDatabaseError> {
        let connect_options = ConnectOptions::new(connection_string).to_owned();

        let conn = sea_orm::Database::connect(connect_options).await?;

        Ok(Self { conn })
    }
}

impl DatabaseService for SqliteDatabase {
    async fn get_serial_port_id(&self, name: &str) -> Result<Option<i32>, GetSerialPortError> {
        tracing::trace!(name = %name, "Getting serial port id");

        let serial_port = entity::serial_port::Entity::find()
            .filter(entity::serial_port::Column::Name.eq(name))
            .into_partial_model::<SerialPortId>()
            .one(&self.conn)
            .await
            .map_err(|err| GetSerialPortError::Get(err.into()))?;

        Ok(serial_port.map(|serial_port| serial_port.id))
    }

    async fn insert_serial_port_returning_id(
        &self,
        name: &str,
    ) -> Result<i32, InsertSerialPortError> {
        tracing::trace!(name = %name, "Inserting serial port");

        let serial_port = entity::serial_port::ActiveModel {
            name: ActiveValue::Set(name.to_owned()),
            ..Default::default()
        };

        let id = serial_port
            .insert(&self.conn)
            .await
            .map_err(|err| InsertSerialPortError::Insert(err.into()))?
            .id;

        Ok(id)
    }

    async fn update_or_insert_serial_port_options_returning_id(
        &self,
        port_id: i32,
        options: AppOpenSerialPortOptions,
    ) -> Result<UpdateOrInsert<i32>, UpdateOrInsertOpenSerialPortOptionsError> {
        tracing::trace!(port_id, "Updating or Inserting open serial port options");

        let options_opt = entity::open_options::Entity::find_by_id(port_id)
            .one(&self.conn)
            .await
            .map_err(|err| UpdateOrInsertOpenSerialPortOptionsError::Update(err.into()))?;

        match options_opt {
            Some(existing_options) => {
                let mut options = entity::open_options::ActiveModel::from((port_id, options));
                options.id = ActiveValue::set(existing_options.id);

                let id = options
                    .update(&self.conn)
                    .await
                    .map_err(|err| UpdateOrInsertOpenSerialPortOptionsError::Update(err.into()))?
                    .id;

                Ok(UpdateOrInsert::Update(id))
            }
            None => {
                let options = entity::open_options::ActiveModel::from((port_id, options));

                let id = options
                    .insert(&self.conn)
                    .await
                    .map_err(|err| UpdateOrInsertOpenSerialPortOptionsError::Insert(err.into()))?
                    .id;

                Ok(UpdateOrInsert::Insert(id))
            }
        }
    }

    async fn insert_packet_returning_id(
        &self,
        port_id: i32,
        tag: String,
        packet: CorePacket,
    ) -> Result<i32, InsertPacketError> {
        tracing::trace!(port_id, %tag, "Inserting packet");

        let packet = entity::packet::ActiveModel::from((port_id, tag, packet));

        let id = packet
            .insert(&self.conn)
            .await
            .map_err(|err| InsertPacketError::Insert(err.into()))?
            .id;

        Ok(id)
    }
}
