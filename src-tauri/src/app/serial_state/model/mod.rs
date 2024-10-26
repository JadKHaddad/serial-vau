use std::time::Duration;

use tokio_util::bytes::Bytes;

#[derive(Debug, Clone)]
pub struct CoreSerialPort {
    pub name: String,
}

impl CoreSerialPort {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

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

/// Defines if an open serial port is currently reading or stopped.
#[derive(Debug, Clone, Copy, Default)]
pub enum CoreReadState {
    #[default]
    Read,
    Stop,
}

/// Defines additional information if the port is in [`Status::Open`] state.
#[derive(Debug)]
pub struct CoreOpenStatus {
    pub read_state: CoreReadState,
}

/// Status of a serial port.
#[derive(Debug)]
pub enum Status {
    Closed,
    Open(CoreOpenStatus),
}

impl CoreReadState {
    pub fn is_stop(&self) -> bool {
        matches!(self, Self::Stop)
    }

    pub fn toggle(self) -> Self {
        match self {
            Self::Read => Self::Stop,
            Self::Stop => Self::Read,
        }
    }
}

#[derive(Debug)]
pub struct CoreManagedSerialPort {
    pub name: String,
    pub status: Status,
    #[cfg(feature = "subscriptions")]
    pub subscriptions: Vec<String>,
    #[cfg(feature = "subscriptions")]
    pub subscribed_to: Vec<String>,
}

impl CoreManagedSerialPort {
    pub fn is_open(&self) -> bool {
        matches!(self.status, Status::Open(_))
    }

    pub fn is_closed(&self) -> bool {
        matches!(self.status, Status::Closed)
    }
}
