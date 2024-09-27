use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

use super::model::packet::Packet;
use crate::core::state::open_serial_port::Packet as CorePacket;

/// Intended to save the packets for serial ports and then dump them to a file if needed.
///
/// ## Note
///
/// - May not be needed for tauri app, since the frontend saves the packets and can send them along a dump command.
/// - Needed for other types of apps, like ratatui or slint.
#[derive(Debug, Default)]
pub struct State {
    packets: RwLock<HashMap<String, Packets>>,
}

impl State {
    /// Get or create a list of packets for a given serial port name.
    pub async fn get_or_create_packets(&self, name: &str) -> Packets {
        let mut packets = self.packets.write().await;

        packets.entry(name.to_string()).or_default().clone()
    }
}

#[derive(Debug, Clone, Default)]
pub struct Packets {
    packets: Arc<RwLock<Vec<Packet>>>,
}

impl Packets {
    pub async fn push(&self, packet: &CorePacket) {
        let packet = Packet {
            packet_direction: packet.packet_direction.clone(),
            timestamp_millis: packet.timestamp_millis,
        };

        self.packets.write().await.push(packet);
    }
}
