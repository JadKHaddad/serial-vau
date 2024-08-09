use crate::{
    app::model::managed_serial_port::ManagedSerialPort,
    core::state::{error::ManagedSerialPortsError, AppState},
};

pub fn subscribe_intern(
    from: &str,
    to: &str,
    _state: &AppState,
) -> Result<Vec<ManagedSerialPort>, SubscribeError> {
    tracing::info!(from=%from, to=%to, "Subscribing");

    #[cfg(feature = "subscriptions")]
    return {
        let managed_serial_ports = _state
            .subscribe(from, to)?
            .into_iter()
            .map(Into::into)
            .collect();

        Ok(managed_serial_ports)
    };

    #[cfg(not(feature = "subscriptions"))]
    Err(SubscribeError::Disabled)
}

pub fn unsubscribe_intern(
    from: &str,
    to: &str,
    _state: &AppState,
) -> Result<Vec<ManagedSerialPort>, SubscribeError> {
    tracing::info!(from=%from, to=%to, "Unsubscribing");

    #[cfg(feature = "subscriptions")]
    return {
        let managed_serial_ports = _state
            .unsubscribe(from, to)?
            .into_iter()
            .map(Into::into)
            .collect();

        Ok(managed_serial_ports)
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
    #[error("Failed to get managed ports: {0}")]
    ManagedSerialPortsError(
        #[source]
        #[from]
        ManagedSerialPortsError,
    ),
}
