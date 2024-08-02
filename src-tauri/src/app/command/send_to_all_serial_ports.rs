use crate::app::state::AppState;

pub fn send_to_all_serial_ports_intern(value: Vec<u8>, state: &AppState) {
    tracing::info!("Sending to all serial ports");

    state.send_to_all_open_serial_ports(value)
}
