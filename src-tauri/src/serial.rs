use serde::Serialize;
use tokio_serial::{Error as TokioSerialError, SerialPortInfo};

#[derive(Debug, Serialize)]
pub struct SerialPortModel {
    name: String,
}

impl SerialPortModel {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl From<SerialPortInfo> for SerialPortModel {
    fn from(value: SerialPortInfo) -> Self {
        Self {
            name: value.port_name,
        }
    }
}

/// Returns a list of all serial ports on system mapped to [`SerialPortModel`].
pub fn available_port_models() -> Result<Vec<SerialPortModel>, TokioSerialError> {
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
            super::available_port_models().expect("Failed to get available ports")
        )
    }
}
