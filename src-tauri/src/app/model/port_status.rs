use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    Closed,
    Open,
}

mod core_impl {
    use super::*;
    use crate::core::serial::managed_serial_port::Status as CoreStatus;

    impl From<CoreStatus> for Status {
        fn from(value: CoreStatus) -> Self {
            match value {
                CoreStatus::Closed => Self::Closed,
                CoreStatus::Open => Self::Open,
            }
        }
    }

    impl From<Status> for CoreStatus {
        fn from(value: Status) -> Self {
            match value {
                Status::Closed => Self::Closed,
                Status::Open => Self::Open,
            }
        }
    }
}
