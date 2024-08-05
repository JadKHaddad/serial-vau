use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadState {
    Read,
    Stop,
}

mod core_impl {
    use super::*;
    use crate::core::serial::managed_serial_port::ReadState as CoreReadState;

    impl From<CoreReadState> for ReadState {
        fn from(value: CoreReadState) -> Self {
            match value {
                CoreReadState::Read => Self::Read,
                CoreReadState::Stop => Self::Stop,
            }
        }
    }

    impl From<ReadState> for CoreReadState {
        fn from(value: ReadState) -> Self {
            match value {
                ReadState::Read => Self::Read,
                ReadState::Stop => Self::Stop,
            }
        }
    }
}
