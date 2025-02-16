use chrono::Local;
use env_logger::{fmt::Color, Builder};
use log::LevelFilter;
use std::io::Write;

pub fn setup_logger(log_level: LevelFilter) -> Result<(), Box<dyn std::error::Error>> {
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
        .init();
    Ok(())
}
