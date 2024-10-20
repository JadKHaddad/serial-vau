use core::time::Duration;

use tokio::sync::{
    mpsc::{error::SendError as TokioSendError, UnboundedSender as MPSCUnboundedSender},
    watch::Sender as WatchSender,
};
use tokio_util::{bytes::Bytes, sync::CancellationToken};

use crate::core::serial::managed_serial_port::CoreReadState;

use super::CoreSerialPort;

#[derive(Debug, Default, Clone)]
pub enum CoreDataBits {
    Five,
    Six,
    Seven,
    #[default]
    Eight,
}

#[derive(Debug, Default, Clone)]
pub enum CoreFlowControl {
    #[default]
    None,
    Software,
    Hardware,
}

#[derive(Debug, Default, Clone)]
pub enum CoreParity {
    #[default]
    None,
    Odd,
    Even,
}

#[derive(Debug, Default, Clone)]
pub enum CoreStopBits {
    #[default]
    One,
    Two,
}

/// Describes how a given serial port should be open.
#[derive(Debug, Default, Clone)]
pub struct CoreOpenSerialPortOptions {
    /// Defines the [`CoreReadState`] of a serial port before it is even open.
    pub initial_read_state: CoreReadState,
    pub baud_rate: u32,
    pub data_bits: CoreDataBits,
    pub flow_control: CoreFlowControl,
    pub parity: CoreParity,
    pub stop_bits: CoreStopBits,
    pub timeout: Duration,
}

/// Represents a packet that is received from a serial port.
#[derive(Debug, Clone, Default)]
pub struct CoreIncomingPacket {
    pub line: Bytes,
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone)]
pub struct CoreSubscriptionPacketOrigin {
    /// The name of the serial port that sent the packet.
    pub name: String,
}

/// Origin of an [`CoreOutgoingPacket`].
#[derive(Debug, Clone, Default)]
pub enum CorePacketOrigin {
    /// Sent directly to the serial port by he user.
    #[default]
    Direct,
    /// Sent via a broadcast to all open serial ports.
    Broadcast,
    /// Sent via a subscription from another serial port.
    #[cfg(feature = "subscriptions")]
    Subscription(CoreSubscriptionPacketOrigin),
}

/// Usefull for tracing.
impl std::fmt::Display for CorePacketOrigin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Direct => write!(f, "Direct"),
            Self::Broadcast => write!(f, "Broadcast"),
            #[cfg(feature = "subscriptions")]
            Self::Subscription(subscription) => {
                write!(f, "Subscription from: [{}]", subscription.name)
            }
        }
    }
}

/// Represents a packet that is sent to a serial port.
#[derive(Debug, Clone, Default)]
pub struct CoreOutgoingPacket {
    /// Bytes sent.
    pub bytes: Bytes,
    /// Origin of an [`CoreOutgoingPacket`].
    pub packet_origin: CorePacketOrigin,
}

#[derive(Debug, Clone)]
pub enum CorePacketDirection {
    /// From the open serial port to the application.
    Incoming(CoreIncomingPacket),
    /// From the application to the open serial port.
    Outgoing(CoreOutgoingPacket),
}

impl Default for CorePacketDirection {
    fn default() -> Self {
        Self::Incoming(CoreIncomingPacket::default())
    }
}

/// Packet emitted by [`CoreSerialState::open_serial_port`](crate::core::state::CoreSerialState::open_serial_port) through the channel.
///
/// Represents a packet that should be sent as single packet and not in a collection.
/// That is why we need the [`Self::port_name`] to know where to send the packet.
#[derive(Debug, Clone, Default)]
pub struct CorePacket {
    pub packet_direction: CorePacketDirection,
    /// The name of the corresponding serial port.
    pub port_name: String,
    pub timestamp_millis: u64,
}

impl CorePacket {
    pub fn new_with_current_timestamp(
        packet_direction: CorePacketDirection,
        port_name: String,
    ) -> Self {
        Self {
            packet_direction,
            port_name,
            timestamp_millis: chrono::Utc::now().timestamp_millis() as u64,
        }
    }
}

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
    use crate::{
        core::serial::managed_serial_port::CoreReadState,
        serial_manager::models::{
            SerialManagerDataBits, SerialManagerFlowControl, SerialManagerOpenSerialPortOptions,
            SerialManagerParity, SerialManagerStopBits,
        },
    };

    use super::{
        CoreDataBits, CoreFlowControl, CoreOpenSerialPortOptions, CoreParity, CoreStopBits,
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
