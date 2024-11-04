use std::future::Future;

use crate::app::{
    model::managed_serial_port::AppOpenSerialPortOptions, serial_state::model::CorePacket,
};

use super::error::{
    GetOrInsertSerialPortError, GetSerialPortError, InsertOpenSerialPortOptionsError,
    InsertPacketError, InsertSerialPortError, UpdateOrInsertOpenSerialPortOptionsError,
};

pub trait DatabaseService {
    fn get_serial_port_id(
        &self,
        name: &str,
    ) -> impl Future<Output = Result<Option<i32>, GetSerialPortError>>;

    fn insert_serial_port_returning_id(
        &self,
        name: &str,
    ) -> impl Future<Output = Result<i32, InsertSerialPortError>>;

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

    fn insert_serial_port_options_returning_id(
        &self,
        port_id: i32,
        options: AppOpenSerialPortOptions,
    ) -> impl Future<Output = Result<i32, InsertOpenSerialPortOptionsError>>;

    fn update_or_insert_serial_port_options_returning_id(
        &self,
        port_id: i32,
        options: AppOpenSerialPortOptions,
    ) -> impl Future<Output = Result<i32, UpdateOrInsertOpenSerialPortOptionsError>>;

    fn insert_packet_returning_id(
        &self,
        port_id: i32,
        tag: String,
        packet: CorePacket,
    ) -> impl Future<Output = Result<i32, InsertPacketError>>;
}
