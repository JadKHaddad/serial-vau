use tokio::io::{AsyncRead, AsyncWrite};
use tokio_serial::{SerialPortBuilderExt, SerialPortInfo};

use crate::serial_manager::{
    error::{SerialManagerAvailablePortsError, SerialManagerOpenPortError},
    model::{SerialManagerOpenSerialPortOptions, SerialManagerPort},
    serial_manager_service::SerialManagerService,
};

#[derive(Debug)]
pub struct TokioSerialManager {
    _private: (),
}

impl TokioSerialManager {
    pub fn new() -> Self {
        tracing::info!("Creating Tokio Serial Manager");

        Self { _private: () }
    }
}

impl Default for TokioSerialManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SerialManagerService for TokioSerialManager {
    fn available_ports(&self) -> Result<Vec<SerialManagerPort>, SerialManagerAvailablePortsError> {
        Ok(tokio_serial::available_ports()
            .map_err(|err| SerialManagerAvailablePortsError::Get(err.into()))?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    fn open_port<'a>(
        &self,
        name: impl Into<std::borrow::Cow<'a, str>>,
        options: SerialManagerOpenSerialPortOptions,
    ) -> Result<impl AsyncRead + AsyncWrite + 'static, SerialManagerOpenPortError> {
        let port = tokio_serial::new(name, options.baud_rate)
            .stop_bits(options.stop_bits.into())
            .data_bits(options.data_bits.into())
            .flow_control(options.flow_control.into())
            .parity(options.parity.into())
            .timeout(options.timeout)
            .open_native_async()
            .map_err(|err| SerialManagerOpenPortError::Open(err.into()))?;

        Ok(port)
    }
}

impl From<SerialPortInfo> for SerialManagerPort {
    fn from(value: SerialPortInfo) -> Self {
        Self::new(value.port_name)
    }
}

mod impl_from {
    use crate::serial_manager::model::{
        SerialManagerDataBits, SerialManagerFlowControl, SerialManagerParity, SerialManagerStopBits,
    };

    use tokio_serial::{
        DataBits as TokioDataBits, FlowControl as TokioFlowControl, Parity as TokioParity,
        StopBits as TokioStopBits,
    };

    impl From<SerialManagerDataBits> for TokioDataBits {
        fn from(data_bits: SerialManagerDataBits) -> Self {
            match data_bits {
                SerialManagerDataBits::Five => TokioDataBits::Five,
                SerialManagerDataBits::Six => TokioDataBits::Six,
                SerialManagerDataBits::Seven => TokioDataBits::Seven,
                SerialManagerDataBits::Eight => TokioDataBits::Eight,
            }
        }
    }

    impl From<SerialManagerFlowControl> for TokioFlowControl {
        fn from(flow_control: SerialManagerFlowControl) -> Self {
            match flow_control {
                SerialManagerFlowControl::None => TokioFlowControl::None,
                SerialManagerFlowControl::Software => TokioFlowControl::Software,
                SerialManagerFlowControl::Hardware => TokioFlowControl::Hardware,
            }
        }
    }

    impl From<SerialManagerParity> for TokioParity {
        fn from(parity: SerialManagerParity) -> Self {
            match parity {
                SerialManagerParity::None => TokioParity::None,
                SerialManagerParity::Odd => TokioParity::Odd,
                SerialManagerParity::Even => TokioParity::Even,
            }
        }
    }

    impl From<SerialManagerStopBits> for TokioStopBits {
        fn from(stop_bits: SerialManagerStopBits) -> Self {
            match stop_bits {
                SerialManagerStopBits::One => TokioStopBits::One,
                SerialManagerStopBits::Two => TokioStopBits::Two,
            }
        }
    }
}
