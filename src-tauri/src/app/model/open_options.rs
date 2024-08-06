use serde::Deserialize;

use super::managed_serial_port::ReadState;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenSerialPortOptions {
    pub name: String,
    pub initial_read_state: ReadState,
}

mod core_impl {
    use super::*;
    use crate::core::state::open_serial_port::OpenSerialPortOptions as CoreOpenSerialPortOptions;

    impl From<OpenSerialPortOptions> for CoreOpenSerialPortOptions {
        fn from(value: OpenSerialPortOptions) -> Self {
            Self {
                name: value.name,
                initial_read_state: value.initial_read_state.into(),
            }
        }
    }
}
