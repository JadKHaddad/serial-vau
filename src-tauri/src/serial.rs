use serde::Serialize;
use tokio_serial::{Error as TokioSerialError, SerialPortInfo};

pub mod watcher;

#[derive(Debug, Serialize, Clone)]
pub struct SerialPort {
    name: String,
}

impl SerialPort {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl From<SerialPortInfo> for SerialPort {
    fn from(value: SerialPortInfo) -> Self {
        Self {
            name: value.port_name,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AvailablePortsError {
    #[error("Failed to get available ports: {0}")]
    SerialError(
        #[source]
        #[from]
        TokioSerialError,
    ),
}

/// Returns a list of all serial ports on system mapped to [`SerialPort`].
pub fn available_ports() -> Result<Vec<SerialPort>, AvailablePortsError> {
    Ok(tokio_serial::available_ports()?
        .into_iter()
        .map(Into::into)
        .collect())
}

#[cfg(test)]
mod test {
    #[test]
    #[ignore]
    fn available_ports() {
        println!(
            "{:?}",
            super::available_ports().expect("Failed to get available ports")
        )
    }
}
