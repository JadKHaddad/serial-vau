use crate::{
    app::model::managed_serial_port::ManagedSerialPort,
    core::state::{error::RemoveOpenSerialPortError, AppState},
};

pub async fn close_serial_port_intern(
    name: String,
    state: &AppState,
) -> Result<Vec<ManagedSerialPort>, RemoveOpenSerialPortError> {
    tracing::info!(name=%name, "Closing serial port");

    let managed_serial_ports = state
        .remove_and_cancel_open_serial_port(&name)?
        .managed_serial_ports
        .into_iter()
        .map(Into::into)
        .collect();

    Ok(managed_serial_ports)
}
