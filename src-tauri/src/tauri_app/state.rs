use crate::app::state::State as AppState;
use crate::core::state::State as SerialState;

#[derive(Debug, Default)]
pub struct State {
    serial_state: SerialState,
    app_state: AppState,
}

impl State {
    pub fn new(serial_state: SerialState, app_state: AppState) -> Self {
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
}
