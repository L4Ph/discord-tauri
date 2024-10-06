#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod tray;

#[cfg(target_os = "windows")]
pub fn run() {
    use tauri::Manager;

  tauri::Builder::default()
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
    .plugin(tauri_plugin_notification::init())
    .invoke_handler(tauri::generate_handler![])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}