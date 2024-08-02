use std::{collections::HashMap, ops::Deref, sync::Arc};

use open_serial_port::{OpenSerialPort, SendError};
use parking_lot::RwLock;

use crate::serial::AvailablePortsError;

use super::model::managed_serial_port::{ManagedSerialPort, Status};

pub mod open_serial_port;

#[derive(Debug, Clone, Default)]
pub struct AppState {
    inner: Arc<AppStateInner>,
}

impl Deref for AppState {
    type Target = AppStateInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[derive(Debug, Default)]
pub struct AppStateInner {
    /// Not using an async `RwLock` because [`WMIConnection`](wmi::WMIConnection) is not [`Send`],
    /// which is used in [`Watcher`](crate::serial::watcher::Watcher),
    /// which is used in [`run`](crate::app::run).
    open_serial_ports: RwLock<HashMap<String, OpenSerialPort>>,
}

impl AppStateInner {
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

    pub fn add_open_serial_port(&self, open_serial_port: OpenSerialPort) -> Option<OpenSerialPort> {
        tracing::debug!(name=%open_serial_port.name(), "Adding serial port");

        self.open_serial_ports
            .write()
            .insert(open_serial_port.name().to_string(), open_serial_port)
    }

    pub fn remove_open_serial_port(&self, name: &str) -> Option<OpenSerialPort> {
        tracing::debug!(name=%name, "Removing serial port");

        self.open_serial_ports.write().remove(name)
    }

    pub fn remove_and_cancel_open_serial_port(&self, name: &str) -> Option<OpenSerialPort> {
        self.remove_open_serial_port(name)
            .map(OpenSerialPort::cancelled)
    }

    /// Ok(Some(bool)) => Port found
    /// Ok(None) => Port not found
    pub fn is_port_open(&self, name: &str) -> Result<Option<bool>, ManagedSerialPortsError> {
        let managed_serial_ports = self.managed_serial_ports()?;
        let managed_serial_port = managed_serial_ports.iter().find(|port| port.name == name);

        Ok(managed_serial_port.map(|port| port.is_open()))
    }

    /// Ok(Some(bool)) => Port found
    /// Ok(None) => Port not found
    pub fn is_port_closed(&self, name: &str) -> Result<Option<bool>, ManagedSerialPortsError> {
        let managed_serial_ports = self.managed_serial_ports()?;
        let managed_serial_port = managed_serial_ports.iter().find(|port| port.name == name);

        Ok(managed_serial_port.map(|port| port.is_closed()))
    }

    /// Some(Ok()) => Ok
    /// Some(Err(_)) => Send error
    /// None => Port not found
    pub fn send_to_open_serial_port(
        &self,
        name: &str,
        value: Vec<u8>,
    ) -> Option<Result<(), SendError>> {
        Some(self.open_serial_ports.read().get(name)?.send(value))
    }

    pub fn send_to_all_open_serial_ports(&self, value: Vec<u8>) {
        self.open_serial_ports.read().values().for_each(|port| {
            let _ = port.send(value.clone());
        })
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
