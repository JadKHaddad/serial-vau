use std::{collections::HashMap, ops::Deref, sync::Arc};

use parking_lot::RwLock;

use super::model::packet::Packet;
use crate::core::state::open_serial_port::Packet as CorePacket;

#[derive(Debug, Clone, Default)]
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
pub struct StateInner {
    packets: RwLock<HashMap<String, Packets>>,
}

impl StateInner {
    /// Get or create a list of packets for a given serial port name.
    pub fn get_or_create_packets(&self, name: &str) -> Packets {
        let mut packets = self.packets.write();

        packets.entry(name.to_string()).or_default().clone()
    }
}

#[derive(Debug, Clone, Default)]
pub struct Packets {
    packets: Arc<RwLock<Vec<Packet>>>,
}

impl Packets {
    pub fn push(&self, packet: &CorePacket) {
        let packet = Packet {
            packet_direction: packet.packet_direction.clone(),
            timestamp_millis: packet.timestamp_millis,
        };

        self.packets.write().push(packet);
    }
}
