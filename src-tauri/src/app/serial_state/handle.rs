use tokio::sync::{
    mpsc::{error::SendError as TokioSendError, UnboundedSender as MPSCUnboundedSender},
    watch::Sender as WatchSender,
};
use tokio_util::sync::CancellationToken;

use super::model::{CoreOutgoingPacket, CoreReadState, CoreSerialPort};

/// Used to copy the [`CoreOpenSerialPort::tx`] field from [`CoreOpenSerialPort`].
/// Used as a handle to send data to a serial port that is a subscriber to another serial port.
#[derive(Debug)]
#[cfg(feature = "subscriptions")]
pub struct TxHandle {
    serial_port: CoreSerialPort,
    tx: MPSCUnboundedSender<CoreOutgoingPacket>,
}

#[cfg(feature = "subscriptions")]
impl TxHandle {
    pub fn send(&self, value: CoreOutgoingPacket) -> Result<(), SendError> {
        Ok(self.tx.send(value)?)
    }

    pub fn name(&self) -> &str {
        self.serial_port.name()
    }
}

#[derive(Debug)]
pub struct CoreOpenSerialPort {
    serial_port: CoreSerialPort,
    /// Main channel to send data to the serial port.
    ///
    /// The write task is waiting for data to be sent to the serial port.
    tx: MPSCUnboundedSender<CoreOutgoingPacket>,
    cancellation_token: CancellationToken,
    /// Defines if the read task is currently reading or stopped.
    ///
    /// The read task is always watching for changes to the read state.
    read_state_tx: WatchSender<CoreReadState>,
}

impl CoreOpenSerialPort {
    pub fn new(
        serial_port: CoreSerialPort,
        tx: MPSCUnboundedSender<CoreOutgoingPacket>,
        cancellation_token: CancellationToken,
        read_state_tx: WatchSender<CoreReadState>,
    ) -> Self {
        Self {
            serial_port,
            tx,
            cancellation_token,
            read_state_tx,
        }
    }

    pub fn name(&self) -> &str {
        self.serial_port.name()
    }

    fn cancel(&self) {
        tracing::debug!(name=%self.name(), "Cancelling");

        self.cancellation_token.cancel()
    }

    pub(super) fn cancelled(self) -> Self {
        self.cancel();
        self
    }

    pub(super) fn send(&self, value: CoreOutgoingPacket) -> Result<(), SendError> {
        Ok(self.tx.send(value)?)
    }

    #[cfg(feature = "subscriptions")]
    pub(super) fn tx_handle(&self) -> TxHandle {
        TxHandle {
            serial_port: self.serial_port.clone(),
            tx: self.tx.clone(),
        }
    }

    /// Fails silently if the send fails. Open serial port is probably closed.
    pub(super) fn set_read_state(&self, read_state: CoreReadState) {
        let _ = self.read_state_tx.send(read_state);
    }

    pub(super) fn read_state(&self) -> CoreReadState {
        *self.read_state_tx.borrow()
    }
}

/// Error returned by [`CoreOpenSerialPort::send`](CoreOpenSerialPort::send) and [`TxHandle::send`](TxHandle::send)
#[derive(Debug, thiserror::Error)]
pub enum SendError {
    #[error("Failed to send: {0}")]
    Send(
        #[source]
        #[from]
        TokioSendError<CoreOutgoingPacket>,
    ),
}

mod impl_from {
    use crate::serial_manager::models::{
        SerialManagerDataBits, SerialManagerFlowControl, SerialManagerOpenSerialPortOptions,
        SerialManagerParity, SerialManagerStopBits,
    };

    use crate::app::serial_state::model::{
        CoreDataBits, CoreFlowControl, CoreOpenSerialPortOptions, CoreParity, CoreReadState,
        CoreStopBits,
    };

    impl From<CoreDataBits> for SerialManagerDataBits {
        fn from(value: CoreDataBits) -> Self {
            match value {
                CoreDataBits::Five => Self::Five,
                CoreDataBits::Six => Self::Six,
                CoreDataBits::Seven => Self::Seven,
                CoreDataBits::Eight => Self::Eight,
            }
        }
    }

    impl From<CoreFlowControl> for SerialManagerFlowControl {
        fn from(value: CoreFlowControl) -> Self {
            match value {
                CoreFlowControl::None => Self::None,
                CoreFlowControl::Software => Self::Software,
                CoreFlowControl::Hardware => Self::Hardware,
            }
        }
    }

    impl From<CoreParity> for SerialManagerParity {
        fn from(value: CoreParity) -> Self {
            match value {
                CoreParity::None => Self::None,
                CoreParity::Odd => Self::Odd,
                CoreParity::Even => Self::Even,
            }
        }
    }

    impl From<CoreStopBits> for SerialManagerStopBits {
        fn from(value: CoreStopBits) -> Self {
            match value {
                CoreStopBits::One => Self::One,
                CoreStopBits::Two => Self::Two,
            }
        }
    }

    impl CoreOpenSerialPortOptions {
        pub fn split_into_read_state_and_manager_options(
            self,
        ) -> (CoreReadState, SerialManagerOpenSerialPortOptions) {
            let CoreOpenSerialPortOptions {
                initial_read_state,
                baud_rate,
                data_bits,
                flow_control,
                parity,
                stop_bits,
                timeout,
            } = self;

            (
                initial_read_state,
                SerialManagerOpenSerialPortOptions {
                    baud_rate,
                    data_bits: data_bits.into(),
                    flow_control: flow_control.into(),
                    parity: parity.into(),
                    stop_bits: stop_bits.into(),
                    timeout,
                },
            )
        }
    }
}
