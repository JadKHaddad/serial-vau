use tauri::{AppHandle, Manager};

use crate::tauri_app::event::events::SERIAL_PACKET_EVENT;

use super::model::packet::PacketEvent;

pub fn emit_packet_event(app: &AppHandle, event: &PacketEvent) -> Result<(), tauri::Error> {
    tracing::debug!("Emitting packet");

    app.emit_all(SERIAL_PACKET_EVENT, &event)
}
