use std::{ops::Deref, sync::Arc};

use error::AddPacketError;

use crate::core::state::open_serial_port::Packet as CorePacket;

pub mod error;

/// Intended to save the packets and open options for serial ports.
#[derive(Debug, Default, Clone)]
pub struct State {
    inner: Arc<StateInner>,
}

impl Deref for State {
    type Target = StateInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[derive(Debug, Default)]
pub struct StateInner {}

impl StateInner {
    /// Add the `packet` to the internal buffer and flush it to the database eventually.
    pub async fn add_packet(&self, packet: &CorePacket) -> Result<(), AddPacketError> {
        Ok(())
    }
}
