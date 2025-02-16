// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::rclone::list_info::*;
use flowcry_lib::log::setup_logger;

mod rclone;

const LOG_LEVEL: log::LevelFilter = log::LevelFilter::Info;

fn main() {
    let app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_remotes, list_files])
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    if let Err(e) = setup_logger(LOG_LEVEL, &app.handle()) {
        eprintln!("Failed to setup logger: {}", e);
    }

    log::info!("Starting Rclone GUI application");

    app.run(|_app_handle, event| match event {
        tauri::RunEvent::Exit => {
            log::info!("Application exiting");
        }
        _ => {}
    });
}
