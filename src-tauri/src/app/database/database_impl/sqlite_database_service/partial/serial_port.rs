use super::super::entity::prelude::SerialPort;
use sea_orm::{DerivePartialModel, FromQueryResult};

#[derive(Debug, DerivePartialModel, FromQueryResult)]
#[sea_orm(entity = "SerialPort")]
pub struct SerialPortId {
    pub id: i32,
}
