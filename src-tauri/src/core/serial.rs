pub mod managed_serial_port;

#[derive(Debug, Clone)]
pub struct CoreSerialPort {
    pub name: String,
}

impl CoreSerialPort {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
