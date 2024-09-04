#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

// Tauri imports
use tauri::{
    Manager
};

mod tray;

#[cfg(target_os = "windows")]
pub fn run() {
  // Start Tauri
  tauri::Builder::default()
    .plugin(tauri_plugin_notification::init())
    .invoke_handler(tauri::generate_handler![])
    .setup(|app| {
        {
            let handle = app.handle();
            tray::create_tray(handle)?;
        }
        if let Some(webview_window) = app.get_webview_window("main") {
            let _ = webview_window.show();
        }
        Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}