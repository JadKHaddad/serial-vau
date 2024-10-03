use crate::{
    app::state::{error::AppManagedSerialPortsError, AppState},
    core::state::CoreSerialState as SerialState,
};

use super::model::managed_serial_port::ManagedSerialPort;

#[derive(Debug, Clone)]
pub struct TauriAppState {
    app_state: AppState,
}

impl TauriAppState {
    pub fn new(app_state: AppState) -> Self {
        Self { app_state }
    }

    pub fn app_state(&self) -> &AppState {
        &self.app_state
    }

    pub fn serial_state(&self) -> &SerialState {
        self.app_state.serial_state()
    }

    pub async fn get_managed_serial_ports(
        &self,
    ) -> Result<Vec<ManagedSerialPort>, AppManagedSerialPortsError> {
        let managed_serial_ports = self.app_state.get_managed_serial_ports().await?;

        tracing::debug!(?managed_serial_ports);

        Ok(managed_serial_ports.into_iter().map(Into::into).collect())
    }
}
