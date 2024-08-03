use crate::app::state::AppState;

pub fn toggle_read_state_intern(name: &str, state: &AppState) -> Result<(), ToggleReadStateError> {
    tracing::info!(name=%name, "Toggling read state");

    state
        .toggle_read_state(name)
        .ok_or(ToggleReadStateError::NotOpen)
}

#[derive(Debug, thiserror::Error)]
pub enum ToggleReadStateError {
    #[error("Port not open")]
    NotOpen,
}
