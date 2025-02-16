use chrono::Local;
use env_logger::{fmt::Color, Builder};
use log::LevelFilter;
use std::fs::File;
use std::io::Write;
use tauri::{AppHandle, Manager};

pub fn setup_logger(
    log_level: LevelFilter,
    app_handle: &AppHandle,
) -> Result<(), Box<dyn std::error::Error>> {
    // 获取应用数据目录
    let app_log_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;

    // 确保日志目录存在
    std::fs::create_dir_all(&app_log_dir)?;

    // 构建日志文件路径
    let log_file = app_log_dir.join("flowcry.log");

    // 创建或打开日志文件
    let file = File::create(&log_file)?;

    Builder::from_default_env()
        .format(|buf, record| {
            let mut style = buf.style();
            let level_color = match record.level() {
                log::Level::Error => Color::Red,
                log::Level::Warn => Color::Yellow,
                log::Level::Info => Color::Green,
                log::Level::Debug => Color::Blue,
                log::Level::Trace => Color::Cyan,
            };
            style.set_color(level_color);
            writeln!(
                buf,
                "{} [{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                style.value(record.level()),
                record.args()
            )
        })
        .filter(None, log_level)
        .target(env_logger::Target::Pipe(Box::new(file)))
        .init();

    log::info!("Logger initialized at: {}", log_file.display());
    Ok(())
}
