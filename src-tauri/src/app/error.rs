use tauri::InvokeError;

#[derive(Debug)]
pub struct AppError {
    err: anyhow::Error,
}

impl<E: Into<anyhow::Error>> From<E> for AppError {
    fn from(value: E) -> Self {
        let err: anyhow::Error = value.into();

        let message = format!("{err:#}");

        tracing::error!(message);

        Self { err }
    }
}

impl From<AppError> for InvokeError {
    fn from(val: AppError) -> Self {
        Self::from_anyhow(val.err)
    }
}
