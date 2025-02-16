use crate::rclone::{get_rclone_path, rclone_cmd, FileItem, RemoteInfo};

// 获取remote列表
#[tauri::command]
pub fn get_remotes() -> Result<Vec<RemoteInfo>, String> {
    let rclone_path = get_rclone_path();
    log::info!("Using rclone at: {}", rclone_path.display());

    log::info!("Executing command: rclone listremotes --long");

    let mut cmd = rclone_cmd();
    cmd.args(["listremotes", "--long"]);

    let output = cmd.output().map_err(|e| {
        let err_msg = format!("Failed to execute rclone listremotes: {}", e);
        log::error!("{}", err_msg);
        err_msg
    })?;

    if !output.status.success() {
        let err_msg = String::from_utf8_lossy(&output.stderr).to_string();
        log::error!("rclone listremotes failed: {}", err_msg);
        return Err(err_msg);
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    let mut remotes = Vec::new();

    for line in output_str.lines() {
        if let Some((name, type_str)) = line.split_once(':') {
            let clean_type = type_str
                .trim()
                .trim_start_matches('[')
                .trim_end_matches(']')
                .trim();

            if clean_type != "local" {
                remotes.push(RemoteInfo {
                    name: name.to_string(),
                    type_: clean_type.to_string(),
                });
            }
        }
    }

    log::info!("Successfully retrieved {} remotes", remotes.len());
    Ok(remotes)
}

// 列出文件和文件夹
#[tauri::command]
pub fn list_files(remote: String, path: String) -> Result<Vec<FileItem>, String> {
    let rclone_path = get_rclone_path();
    log::info!("Listing files for remote '{}' at path '{}'", remote, path);

    let mut items = Vec::new();

    // 获取文件夹
    log::info!("Executing command: rclone lsd {}:{}", remote, path);

    let mut cmd = rclone_cmd();
    cmd.args(["lsd", &format!("{}:{}", remote, path)]);

    let dirs_output = cmd.output().map_err(|e| {
        let err_msg = format!("Failed to execute rclone lsd: {}", e);
        log::error!("{}", err_msg);
        err_msg
    })?;

    if dirs_output.status.success() {
        let dirs_str = String::from_utf8_lossy(&dirs_output.stdout);
        for line in dirs_str.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                items.push(FileItem {
                    name: parts.last().unwrap().to_string(),
                    is_dir: true,
                });
            }
        }
        log::info!("Found {} directories", items.len());
    } else {
        log::error!(
            "rclone lsd failed: {}",
            String::from_utf8_lossy(&dirs_output.stderr)
        );
    }

    // 获取文件
    log::info!("Executing command: rclone ls {}:{}", remote, path);
    let mut cmd = rclone_cmd();
    cmd.args(["ls", &format!("{}:{}", remote, path)]);

    let files_output = cmd.output().map_err(|e| {
        let err_msg = format!("Failed to execute rclone ls: {}", e);
        log::error!("{}", err_msg);
        err_msg
    })?;

    if files_output.status.success() {
        let files_str = String::from_utf8_lossy(&files_output.stdout);
        let initial_count = items.len();
        for line in files_str.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                items.push(FileItem {
                    name: parts.last().unwrap().to_string(),
                    is_dir: false,
                });
            }
        }
        log::info!("Found {} files", items.len() - initial_count);
    } else {
        log::error!(
            "rclone ls failed: {}",
            String::from_utf8_lossy(&files_output.stderr)
        );
    }

    log::info!("Total items found: {}", items.len());
    Ok(items)
}
