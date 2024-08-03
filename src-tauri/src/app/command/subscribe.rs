use crate::app::state::AppState;

pub fn subscribe_intern(from: &str, to: &str, state: &AppState) {
    tracing::info!(from=%from, to=%to, "Subscribing");

    state.subscribe(from, to)
}

pub fn unsubscribe_intern(from: &str, to: &str, state: &AppState) {
    tracing::info!(from=%from, to=%to, "Unsubscribing");

    state.unsubscribe(from, to)
}
