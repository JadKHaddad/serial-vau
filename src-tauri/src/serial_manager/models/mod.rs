use std::time::Duration;

#[derive(Debug)]
pub struct SerialManagerPort {
    pub name: String,
}

impl SerialManagerPort {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[derive(Debug)]
pub enum SerialManagerDataBits {
    Five,
    Six,
    Seven,
    Eight,
}

#[derive(Debug)]
pub enum SerialManagerFlowControl {
    None,
    Software,
    Hardware,
}

#[derive(Debug)]
pub enum SerialManagerParity {
    None,
    Odd,
    Even,
}

#[derive(Debug)]
pub enum SerialManagerStopBits {
    One,
    Two,
}

#[derive(Debug)]
pub struct SerialManagerOpenSerialPortOptions {
    pub baud_rate: u32,
    pub data_bits: SerialManagerDataBits,
    pub flow_control: SerialManagerFlowControl,
    pub parity: SerialManagerParity,
    pub stop_bits: SerialManagerStopBits,
    pub timeout: Duration,
}
