use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorEvent {
    pub error: String,
}

impl<E> From<E> for ErrorEvent
where
    E: std::error::Error,
{
    fn from(error: E) -> Self {
        Self {
            error: error.to_string(),
        }
    }
}
