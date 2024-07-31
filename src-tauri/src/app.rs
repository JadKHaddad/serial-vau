use anyhow::Context;

pub mod state;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}", name)
}

pub fn run() -> anyhow::Result<()> {
    tauri::Builder::default()
        .setup(|_app| {
            let _ = crate::wmi::spawn_serial_events_watchers(|_e| {}, |_e| {});

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .context("Error while running tauri application")
}
