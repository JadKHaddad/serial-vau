/// Contains the options that can are database models as well.
/// These fields are not from [OpenSerialPortOptions](crate::core::state::open_serial_port::OpenSerialPortOptions).
#[derive(Debug, Default, Clone)]
pub struct OpenSerialPortOptions {
    // pub baud_rate: u32,
    // pub data_bits: DataBits,
    // pub flow_control: FlowControl,
    // pub parity: Parity,
    // pub stop_bits: StopBits,
    // pub timeout: Duration,
}

mod core_impl {
    use super::*;
    use crate::core::state::open_serial_port::CoreOpenSerialPortOptions as CoreOpenSerialPortOptions;

    impl From<OpenSerialPortOptions> for CoreOpenSerialPortOptions {
        fn from(value: OpenSerialPortOptions) -> Self {
            CoreOpenSerialPortOptions::default()
        }
    }
}
