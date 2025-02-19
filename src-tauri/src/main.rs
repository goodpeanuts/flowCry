// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::OnceLock;

use crate::rclone::list_info::*;
use flowcry_lib::log::setup_logger;
use flowcry_lib::rclone;
use flowcry_lib::set_app_handle;
use tauri::AppHandle;

const LOG_LEVEL: log::LevelFilter = log::LevelFilter::Info;

// 创建一个全局静态变量来存储 AppHandle
static GLOBAL_APP_HANDLE: OnceLock<AppHandle> = OnceLock::new();

fn main() {
    let app = tauri::Builder::default()
        .setup(|app| {
            set_app_handle(app.handle().clone());
            rclone::init_rclone_path(app.handle());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_remotes, list_files])
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    if let Err(e) = setup_logger(LOG_LEVEL, app.handle()) {
        eprintln!("Failed to setup logger: {}", e);
    }

    log::info!("Starting Rclone GUI application");

    app.run(|_app_handle, event| {
        if let tauri::RunEvent::Exit = event {
            log::info!("Application exiting");
        }
    });
}

// 创建一个辅助函数来获取 AppHandle
pub fn get_app_handle() -> &'static AppHandle {
    GLOBAL_APP_HANDLE.get().expect("AppHandle not initialized")
}
