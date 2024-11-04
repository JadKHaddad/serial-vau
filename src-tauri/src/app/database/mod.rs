use crate::app::{
    database::{
        database_impl::sqlite_database_service::SqliteDatabase, database_service::DatabaseService,
        error::*, model::UpdateOrInsert,
    },
    model::managed_serial_port::AppOpenSerialPortOptions,
    serial_state::model::CorePacket,
};

pub mod database_impl;
pub mod database_service;
pub mod error;
pub mod model;

#[derive(Debug)]
#[enum_dispatch::enum_dispatch(DatabaseService)]
pub enum Database {
    SqliteDatabase,
}
