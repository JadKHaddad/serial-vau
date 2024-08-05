use serde::Serialize;

use super::{port_status::Status, read_state::ReadState};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagedSerialPort {
    pub name: String,
    pub status: Status,
    #[cfg(feature = "subscriptions")]
    pub subscriptions: Vec<String>,
    #[cfg(feature = "subscriptions")]
    pub subscribed_to: Vec<String>,
    pub read_state: Option<ReadState>,
}

mod core_impl {
    use super::*;
    use crate::core::serial::managed_serial_port::ManagedSerialPort as CoreManagedSerialPort;

    impl From<CoreManagedSerialPort> for ManagedSerialPort {
        fn from(value: CoreManagedSerialPort) -> Self {
            Self {
                name: value.name,
                status: value.status.into(),
                #[cfg(feature = "subscriptions")]
                subscriptions: value.subscriptions,
                #[cfg(feature = "subscriptions")]
                subscribed_to: value.subscribed_to,
                read_state: value.read_state.map(Into::into),
            }
        }
    }
}
