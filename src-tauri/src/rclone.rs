use serde::Serialize;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::OnceLock;
use tauri::{AppHandle, Manager};

pub mod list_info;

#[derive(Serialize)]
pub struct FileItem {
    name: String,
    is_dir: bool,
}

#[derive(Serialize)]
pub struct RemoteInfo {
    name: String,
    type_: String, // 使用type_因为type是保留字
}

static RCLONE_PATH: OnceLock<PathBuf> = OnceLock::new();

pub fn init_rclone_path(app: &AppHandle) {
    let _ = RCLONE_PATH.get_or_init(|| get_rclone_executable_path(app));
}

fn get_rclone_path() -> &'static PathBuf {
    let rclone_path = RCLONE_PATH
        .get()
        .expect("Rclone path not initialized. Call init_rclone_path first");
    log::info!("Using rclone at: {}", rclone_path.display());
    rclone_path
}

pub fn rclone_cmd() -> Command {
    let rclone_path = get_rclone_path();
    let mut cmd = Command::new(rclone_path);

    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x08000000);
        cmd.current_dir("D:\\");
    }

    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());
    cmd
}

#[cfg(not(target_os = "windows"))]
pub fn configure_command(_cmd: &mut Command) {}

fn get_rclone_executable_path(app: &AppHandle) -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        app.path()
            .resource_dir()
            .unwrap()
            .join("resources/rclone.exe")
    }
    #[cfg(not(target_os = "windows"))]
    {
        app.path().resource_dir().unwrap().join("resources/rclone")
    }
}
