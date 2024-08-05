use serde::Serialize;

use crate::app::model::managed_serial_port::ManagedSerialPort;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagedSerialPorts {
    pub ports: Vec<ManagedSerialPort>,
}
