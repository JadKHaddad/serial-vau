use crate::{
    app::model::managed_serial_port::ManagedSerialPort,
    core::state::{error::ToggleReadStateError, AppState},
};

pub fn toggle_read_state_intern(
    name: &str,
    state: &AppState,
) -> Result<Vec<ManagedSerialPort>, ToggleReadStateError> {
    tracing::info!(name=%name, "Toggling read state");

    let managed_serial_ports = state
        .toggle_read_state(name)?
        .into_iter()
        .map(Into::into)
        .collect();

    Ok(managed_serial_ports)
}
