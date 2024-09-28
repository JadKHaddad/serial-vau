use serde::{Deserialize, Serialize};

use super::managed_serial_port::ReadState;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DataBits {
    Five,
    Six,
    Seven,
    Eight,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum FlowControl {
    None,
    Software,
    Hardware,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Parity {
    None,
    Odd,
    Even,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum StopBits {
    One,
    Two,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Duration {
    pub secs: u64,
    pub nanos: u32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenSerialPortOptions {
    pub initial_read_state: ReadState,
    pub baud_rate: u32,
    pub data_bits: DataBits,
    pub flow_control: FlowControl,
    pub parity: Parity,
    pub stop_bits: StopBits,
    pub timeout: Duration,
}

mod core_impl {
    use super::*;

    use crate::core::state::open_serial_port::{
        DataBits as CoreDataBits, FlowControl as CoreFlowControl,
        OpenSerialPortOptions as CoreOpenSerialPortOptions, Parity as CoreParity,
        StopBits as CoreStopBits,
    };
    use core::time::Duration as CoreDuration;

    impl From<DataBits> for CoreDataBits {
        fn from(value: DataBits) -> Self {
            match value {
                DataBits::Five => Self::Five,
                DataBits::Six => Self::Six,
                DataBits::Seven => Self::Seven,
                DataBits::Eight => Self::Eight,
            }
        }
    }

    impl From<CoreDataBits> for DataBits {
        fn from(value: CoreDataBits) -> Self {
            match value {
                CoreDataBits::Five => Self::Five,
                CoreDataBits::Six => Self::Six,
                CoreDataBits::Seven => Self::Seven,
                CoreDataBits::Eight => Self::Eight,
            }
        }
    }

    impl From<FlowControl> for CoreFlowControl {
        fn from(value: FlowControl) -> Self {
            match value {
                FlowControl::None => Self::None,
                FlowControl::Software => Self::Software,
                FlowControl::Hardware => Self::Hardware,
            }
        }
    }

    impl From<CoreFlowControl> for FlowControl {
        fn from(value: CoreFlowControl) -> Self {
            match value {
                CoreFlowControl::None => Self::None,
                CoreFlowControl::Software => Self::Software,
                CoreFlowControl::Hardware => Self::Hardware,
            }
        }
    }

    impl From<Parity> for CoreParity {
        fn from(value: Parity) -> Self {
            match value {
                Parity::None => Self::None,
                Parity::Odd => Self::Odd,
                Parity::Even => Self::Even,
            }
        }
    }

    impl From<CoreParity> for Parity {
        fn from(value: CoreParity) -> Self {
            match value {
                CoreParity::None => Self::None,
                CoreParity::Odd => Self::Odd,
                CoreParity::Even => Self::Even,
            }
        }
    }

    impl From<StopBits> for CoreStopBits {
        fn from(value: StopBits) -> Self {
            match value {
                StopBits::One => Self::One,
                StopBits::Two => Self::Two,
            }
        }
    }

    impl From<CoreStopBits> for StopBits {
        fn from(value: CoreStopBits) -> Self {
            match value {
                CoreStopBits::One => Self::One,
                CoreStopBits::Two => Self::Two,
            }
        }
    }

    impl From<Duration> for CoreDuration {
        fn from(value: Duration) -> Self {
            Self::new(value.secs, value.nanos)
        }
    }

    impl From<CoreDuration> for Duration {
        fn from(value: CoreDuration) -> Self {
            Self {
                secs: value.as_secs(),
                nanos: value.subsec_nanos(),
            }
        }
    }

    impl From<OpenSerialPortOptions> for CoreOpenSerialPortOptions {
        fn from(value: OpenSerialPortOptions) -> Self {
            Self {
                initial_read_state: value.initial_read_state.into(),
                baud_rate: value.baud_rate,
                data_bits: value.data_bits.into(),
                flow_control: value.flow_control.into(),
                parity: value.parity.into(),
                stop_bits: value.stop_bits.into(),
                timeout: value.timeout.into(),
            }
        }
    }

    impl From<CoreOpenSerialPortOptions> for OpenSerialPortOptions {
        fn from(value: CoreOpenSerialPortOptions) -> Self {
            Self {
                initial_read_state: value.initial_read_state.into(),
                baud_rate: value.baud_rate,
                data_bits: value.data_bits.into(),
                flow_control: value.flow_control.into(),
                parity: value.parity.into(),
                stop_bits: value.stop_bits.into(),
                timeout: value.timeout.into(),
            }
        }
    }
}
