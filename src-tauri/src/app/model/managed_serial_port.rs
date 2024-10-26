use crate::app::serial_state::model::{CoreManagedSerialPort, CoreOpenSerialPortOptions};

#[derive(Debug)]
pub struct AppManagedSerialPort {
    pub managed_serial_port: CoreManagedSerialPort,
    pub last_used_open_options: AppOpenSerialPortOptions,
}

#[derive(Debug, Default, Clone)]
pub struct AppOpenSerialPortOptions {
    pub tag: String,
    pub core_options: CoreOpenSerialPortOptions,
}
