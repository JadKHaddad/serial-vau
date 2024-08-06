use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadState {
    Read,
    Stop,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenStatus {
    pub read_state: ReadState,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type", content = "content")]
pub enum Status {
    Closed,
    Open(OpenStatus),
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagedSerialPort {
    pub name: String,
    pub status: Status,
    #[cfg(feature = "subscriptions")]
    pub subscriptions: Vec<String>,
    #[cfg(feature = "subscriptions")]
    pub subscribed_to: Vec<String>,
}

mod core_impl {
    use super::*;
    use crate::core::serial::managed_serial_port::ManagedSerialPort as CoreManagedSerialPort;
    use crate::core::serial::managed_serial_port::OpenStatus as CoreOpenStatus;
    use crate::core::serial::managed_serial_port::ReadState as CoreReadState;
    use crate::core::serial::managed_serial_port::Status as CoreStatus;

    impl From<CoreOpenStatus> for OpenStatus {
        fn from(value: CoreOpenStatus) -> Self {
            Self {
                read_state: value.read_state.into(),
            }
        }
    }

    impl From<OpenStatus> for CoreOpenStatus {
        fn from(value: OpenStatus) -> Self {
            Self {
                read_state: value.read_state.into(),
            }
        }
    }

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

    impl From<CoreStatus> for Status {
        fn from(value: CoreStatus) -> Self {
            match value {
                CoreStatus::Closed => Self::Closed,
                CoreStatus::Open(open_status) => Self::Open(open_status.into()),
            }
        }
    }

    impl From<Status> for CoreStatus {
        fn from(value: Status) -> Self {
            match value {
                Status::Closed => Self::Closed,
                Status::Open(open_status) => Self::Open(open_status.into()),
            }
        }
    }

    impl From<CoreManagedSerialPort> for ManagedSerialPort {
        fn from(value: CoreManagedSerialPort) -> Self {
            Self {
                name: value.name,
                status: value.status.into(),
                #[cfg(feature = "subscriptions")]
                subscriptions: value.subscriptions,
                #[cfg(feature = "subscriptions")]
                subscribed_to: value.subscribed_to,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "Only used for manual verification"]
    fn serialize_and_print_open_managed_serial_port() {
        let managed_serial_port = ManagedSerialPort {
            name: "COM1".to_string(),
            status: Status::Open(OpenStatus {
                read_state: ReadState::Read,
            }),
            #[cfg(feature = "subscriptions")]
            subscriptions: vec!["COM2".to_string()],
            #[cfg(feature = "subscriptions")]
            subscribed_to: vec!["COM3".to_string()],
        };

        let serialized = serde_json::to_string_pretty(&managed_serial_port).unwrap();

        println!("{}", serialized);
    }

    #[test]
    #[ignore = "Only used for manual verification"]
    fn serialize_and_print_closed_managed_serial_port() {
        let managed_serial_port = ManagedSerialPort {
            name: "COM1".to_string(),
            status: Status::Closed,
            #[cfg(feature = "subscriptions")]
            subscriptions: vec!["COM2".to_string()],
            #[cfg(feature = "subscriptions")]
            subscribed_to: vec!["COM3".to_string()],
        };

        let serialized = serde_json::to_string_pretty(&managed_serial_port).unwrap();

        println!("{}", serialized);
    }
}
