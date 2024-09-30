use std::time::Duration;

use sea_orm::ActiveValue;

use crate::{
    app::model::managed_serial_port::AppOpenSerialPortOptions,
    core::{
        serial::managed_serial_port::CoreReadState,
        state::open_serial_port::{
            CoreDataBits, CoreFlowControl, CoreOpenSerialPortOptions, CoreParity, CoreStopBits,
        },
    },
};

use super::entity::open_options::{
    ActiveModel as OpenOptionsActiveModel, Model as OpenOptionsModel,
};

impl From<OpenOptionsModel> for AppOpenSerialPortOptions {
    fn from(model: OpenOptionsModel) -> Self {
        Self {
            tag: model.tag,
            core_options: CoreOpenSerialPortOptions {
                initial_read_state: match model.init_read_state {
                    0 => CoreReadState::Read,
                    1 => CoreReadState::Stop,
                    _ => Default::default(),
                },
                baud_rate: model.baud_rate as u32,
                data_bits: match model.data_bits {
                    0 => CoreDataBits::Five,
                    1 => CoreDataBits::Six,
                    2 => CoreDataBits::Seven,
                    3 => CoreDataBits::Eight,
                    _ => Default::default(),
                },
                flow_control: match model.flow_control {
                    0 => CoreFlowControl::None,
                    1 => CoreFlowControl::Software,
                    2 => CoreFlowControl::Hardware,
                    _ => Default::default(),
                },
                parity: match model.parity {
                    0 => CoreParity::None,
                    1 => CoreParity::Odd,
                    2 => CoreParity::Even,
                    _ => Default::default(),
                },
                stop_bits: match model.stop_bits {
                    0 => CoreStopBits::One,
                    1 => CoreStopBits::Two,
                    _ => Default::default(),
                },
                timeout: Duration::from_millis(model.timeout_milli_secs as u64),
            },
        }
    }
}

impl From<AppOpenSerialPortOptions> for OpenOptionsActiveModel {
    fn from(options: AppOpenSerialPortOptions) -> Self {
        Self {
            tag: ActiveValue::Set("TODO".to_string()),
            init_read_state: ActiveValue::Set(match options.core_options.initial_read_state {
                CoreReadState::Read => 0,
                CoreReadState::Stop => 1,
            }),
            baud_rate: ActiveValue::Set(options.core_options.baud_rate as i32),
            data_bits: ActiveValue::Set(match options.core_options.data_bits {
                CoreDataBits::Five => 0,
                CoreDataBits::Six => 1,
                CoreDataBits::Seven => 2,
                CoreDataBits::Eight => 3,
            }),
            flow_control: ActiveValue::Set(match options.core_options.flow_control {
                CoreFlowControl::None => 0,
                CoreFlowControl::Software => 1,
                CoreFlowControl::Hardware => 2,
            }),
            parity: ActiveValue::Set(match options.core_options.parity {
                CoreParity::None => 0,
                CoreParity::Odd => 1,
                CoreParity::Even => 2,
            }),
            stop_bits: ActiveValue::Set(match options.core_options.stop_bits {
                CoreStopBits::One => 0,
                CoreStopBits::Two => 1,
            }),
            timeout_milli_secs: ActiveValue::Set(options.core_options.timeout.as_millis() as i32),
            ..Default::default()
        }
    }
}

// TODO impl packet From/To CorePacket
