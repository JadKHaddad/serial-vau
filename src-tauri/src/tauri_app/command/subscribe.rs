use crate::tauri_app::{
    model::managed_serial_port::ManagedSerialPort,
    state::{TauriAppState, TauriAppStateManagedSerialPortsError},
};

pub async fn subscribe_intern(
    from: &str,
    to: &str,
    _state: &TauriAppState,
) -> Result<Vec<ManagedSerialPort>, SubscribeError> {
    tracing::info!(from=%from, to=%to, "Subscribing");

    #[cfg(feature = "subscriptions")]
    return {
        _state.serial_state().subscribe(from, to).await;

        let managed_serial_ports = _state.get_managed_serial_ports().await?;

        Ok(managed_serial_ports)
    };

    #[cfg(not(feature = "subscriptions"))]
    Err(SubscribeError::Disabled)
}

pub async fn unsubscribe_intern(
    from: &str,
    to: &str,
    _state: &TauriAppState,
) -> Result<Vec<ManagedSerialPort>, SubscribeError> {
    tracing::info!(from=%from, to=%to, "Unsubscribing");

    #[cfg(feature = "subscriptions")]
    return {
        _state.serial_state().unsubscribe(from, to).await;

        let managed_serial_ports = _state.get_managed_serial_ports().await?;

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
        TauriAppStateManagedSerialPortsError,
    ),
}
