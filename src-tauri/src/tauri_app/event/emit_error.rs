use tauri::{AppHandle, Manager};

use crate::tauri_app::event::events::ERROR_EVENT;

use super::model::error::ErrorEvent;

pub fn emit_error_event(app: &AppHandle, event: &ErrorEvent) -> Result<(), tauri::Error> {
    tracing::debug!("Emitting error");

    app.emit_all(ERROR_EVENT, &event)
}
