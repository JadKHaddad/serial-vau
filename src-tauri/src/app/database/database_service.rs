use crate::app::{
    model::managed_serial_port::AppOpenSerialPortOptions, serial_state::model::CorePacket,
};

use super::{error::*, model::UpdateOrInsert};

#[enum_dispatch::enum_dispatch]
pub trait DatabaseService {
    async fn get_serial_port_id(&self, name: &str) -> Result<Option<i32>, GetSerialPortError>;

    async fn insert_serial_port_returning_id(
        &self,
        name: &str,
    ) -> Result<i32, InsertSerialPortError>;

    async fn get_serial_port_id_or_insert_returning_id(
        &self,
        name: &str,
    ) -> Result<i32, GetOrInsertSerialPortError> {
        let id = self.get_serial_port_id(name).await?;

        let id = match id {
            Some(id) => id,
            None => self.insert_serial_port_returning_id(name).await?,
        };

        Ok(id)
    }

    async fn update_or_insert_serial_port_options_returning_id(
        &self,
        port_id: i32,
        options: AppOpenSerialPortOptions,
    ) -> Result<UpdateOrInsert<i32>, UpdateOrInsertOpenSerialPortOptionsError>;

    async fn insert_packet_returning_id(
        &self,
        port_id: i32,
        tag: String,
        packet: CorePacket,
    ) -> Result<i32, InsertPacketError>;
}
