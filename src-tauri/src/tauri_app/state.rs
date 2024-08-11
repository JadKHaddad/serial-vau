use std::{ops::Deref, sync::Arc};

use crate::app::state::State as AppState;
use crate::core::state::State as SerialState;

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

impl State {
    pub fn new(serial_state: SerialState, app_state: AppState) -> Self {
        Self {
            inner: Arc::new(StateInner {
                serial_state,
                app_state,
            }),
        }
    }
}

#[derive(Debug, Default)]
pub struct StateInner {
    serial_state: SerialState,
    app_state: AppState,
}

impl StateInner {
    pub fn serial_state(&self) -> &SerialState {
        &self.serial_state
    }

    pub fn app_state(&self) -> &AppState {
        &self.app_state
    }
}
