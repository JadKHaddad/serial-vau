use std::{collections::HashMap, sync::Arc};

use parking_lot::RwLock;
use tokio::sync::mpsc::Sender;
use tokio_util::sync::CancellationToken;

use crate::serial::{AvailablePortsError, SerialPort};

use super::model::managed_serial_port::{ManagedSerialPort, Status};

#[derive(Debug)]
pub struct OpenSerialPort {
    serial_port: SerialPort,
    tx: Sender<String>,
    cancellation_token: CancellationToken,
}

#[derive(Debug, Clone, Default)]
pub struct AppState {
    open_serial_ports: Arc<RwLock<HashMap<String, OpenSerialPort>>>,
}

impl AppState {
    pub fn managed_serial_ports(&self) -> Result<Vec<ManagedSerialPort>, ManagedSerialPortsError> {
        let available_serial_ports = crate::serial::available_ports()?;
        let open_serial_ports = self.open_serial_ports.read();
        let managed_serial_ports = available_serial_ports
            .into_iter()
            .map(|port| {
                let mut managed_serial_port = ManagedSerialPort {
                    name: port.name().to_string(),
                    status: Status::Closed,
                };

                if open_serial_ports.contains_key(port.name()) {
                    managed_serial_port.status = Status::Open
                }

                managed_serial_port
            })
            .collect::<Vec<_>>();

        Ok(managed_serial_ports)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ManagedSerialPortsError {
    #[error("Failed to get available ports: {0}")]
    AvailablePortsError(
        #[source]
        #[from]
        AvailablePortsError,
    ),
}
