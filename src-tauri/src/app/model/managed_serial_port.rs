use crate::core::{
    serial::managed_serial_port::CoreManagedSerialPort,
    state::open_serial_port::CoreOpenSerialPortOptions,
};

#[derive(Debug)]
pub struct AppManagedSerialPort {
    pub managed_serial_port: CoreManagedSerialPort,
    pub last_used_open_options: CoreOpenSerialPortOptions,
}
