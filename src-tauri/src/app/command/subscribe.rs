use crate::core::state::AppState;

pub fn subscribe_intern(from: &str, to: &str, _state: &AppState) -> Result<(), SubscribeError> {
    tracing::info!(from=%from, to=%to, "Subscribing");

    #[cfg(feature = "subscriptions")]
    return {
        _state.subscribe(from, to);
        Ok(())
    };

    #[cfg(not(feature = "subscriptions"))]
    Err(SubscribeError::Disabled)
}

pub fn unsubscribe_intern(from: &str, to: &str, _state: &AppState) -> Result<(), SubscribeError> {
    tracing::info!(from=%from, to=%to, "Unsubscribing");

    #[cfg(feature = "subscriptions")]
    return {
        _state.unsubscribe(from, to);
        Ok(())
    };

    #[cfg(not(feature = "subscriptions"))]
    Err(SubscribeError::Disabled)
}

#[derive(Debug, thiserror::Error)]
pub enum SubscribeError {
    #[cfg(not(feature = "subscriptions"))]
    #[error(
        "Subscriptions feature is not enabled, turn on the `subscriptions` feature to enable it"
    )]
    Disabled,
}
