// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub mod log;
pub mod rclone;

use std::sync::OnceLock;
use tauri::AppHandle;

// Add at the top level of lib.rs
static GLOBAL_APP_HANDLE: OnceLock<AppHandle> = OnceLock::new();

pub fn get_app_handle() -> &'static AppHandle {
    GLOBAL_APP_HANDLE.get().expect("AppHandle not initialized")
}

pub fn set_app_handle(handle: AppHandle) {
    GLOBAL_APP_HANDLE
        .set(handle)
        .expect("Failed to set global app handle");
}
