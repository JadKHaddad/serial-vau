use std::{ops::Deref, sync::Arc};

use crate::{
    app::state::{error::AppGetOpenSerialPortOptionsError, State as AppState},
    core::state::{
        error::CoreManagedSerialPortsError, open_serial_port::CoreOpenSerialPortOptions,
        State as SerialState,
    },
};

use super::model::managed_serial_port::ManagedSerialPort;

// TODO: move the logic from this State to app::State and make SerialState a private part of app::State.

#[derive(Debug, Default, Clone)]
pub struct TauriAppState {
    inner: Arc<StateInner>,
}

impl TauriAppState {
    pub fn new(serial_state: SerialState, app_state: AppState) -> Self {
        Self {
            inner: Arc::new(StateInner::new(serial_state, app_state)),
        }
    }
}

impl Deref for TauriAppState {
    type Target = StateInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[derive(Debug, Default)]
pub struct StateInner {
    serial_state: SerialState,
    app_state: AppState,
}

impl StateInner {
    fn new(serial_state: SerialState, app_state: AppState) -> Self {
        Self {
            serial_state,
            app_state,
        }
    }

    pub fn serial_state(&self) -> &SerialState {
        &self.serial_state
    }

    pub fn app_state(&self) -> &AppState {
        &self.app_state
    }

    pub async fn get_managed_serial_ports(
        &self,
    ) -> Result<Vec<ManagedSerialPort>, TauriAppStateManagedSerialPortsError> {
        let managed_serial_ports = self.serial_state().managed_serial_ports().await?;
        let open_serial_port_options = self.app_state().get_all_open_serial_port_options().await?;
        let managed_serial_ports = managed_serial_ports
            .into_iter()
            .map(|port| {
                let last_used_open_options = open_serial_port_options
                    .get(&port.name)
                    .cloned()
                    .unwrap_or_default();

                ManagedSerialPort::from((
                    port,
                    CoreOpenSerialPortOptions::from(last_used_open_options),
                ))
            })
            .collect();

        Ok(managed_serial_ports)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TauriAppStateManagedSerialPortsError {
    #[error("Failed to get managed ports: {0}")]
    ManagedSerialPortsError(
        #[source]
        #[from]
        CoreManagedSerialPortsError,
    ),
    #[error("Failed to get open serial port options: {0}")]
    GetOpenSerialPortOptionsError(
        #[source]
        #[from]
        AppGetOpenSerialPortOptionsError,
    ),
}
