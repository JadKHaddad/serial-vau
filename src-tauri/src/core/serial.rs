use tokio_serial::{Error as TokioSerialError, SerialPortInfo};

pub mod managed_serial_port;
pub mod watcher;

#[derive(Debug, Clone)]
pub struct CoreSerialPort {
    name: String,
}

impl CoreSerialPort {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl From<SerialPortInfo> for CoreSerialPort {
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

/// Returns a list of all serial ports on system mapped to [`CoreSerialPort`].
pub fn available_ports() -> Result<Vec<CoreSerialPort>, AvailablePortsError> {
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
