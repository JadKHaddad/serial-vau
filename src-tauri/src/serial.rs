use serde::Serialize;
use tokio_serial::{Error as TokioSerialError, SerialPortInfo};

#[derive(Debug, Serialize)]
pub struct SerialPort {
    name: String,
}

impl From<SerialPortInfo> for SerialPort {
    fn from(value: SerialPortInfo) -> Self {
        Self {
            name: value.port_name,
        }
    }
}

/// Returns a list of all serial ports on system mapped to [`SerialPort`].
pub fn available_ports() -> Result<Vec<SerialPort>, TokioSerialError> {
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
