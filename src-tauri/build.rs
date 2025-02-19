use std::{env, fs};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let current_dir = env::current_dir().expect("Failed to get current directory");
    let out_dir = current_dir.join("resources");

    if let Err(e) = fs::create_dir_all(&out_dir) {
        println!("cargo:warning=Failed to create resources directory: {}", e);
        return;
    }

    #[cfg(target_os = "macos")]
    {
        let source = "/opt/homebrew/bin/rclone";
        let target = out_dir.join("rclone");

        if !target.exists() {
            println!(
                "cargo:warning=Copying rclone from {} to {}",
                source,
                target.display()
            );
            match fs::copy(source, &target) {
                Ok(_) => {
                    // 设置执行权限
                    use std::os::unix::fs::PermissionsExt;
                    if let Ok(mut perms) = fs::metadata(&target).map(|m| m.permissions()) {
                        perms.set_mode(0o755);
                        let _ = fs::set_permissions(&target, perms);
                    }
                }
                Err(e) => println!("cargo:warning=Failed to copy rclone: {}", e),
            }
        }
    }
    #[cfg(target_os = "windows")]
    {
        let from = Command::new("where.exe")
            .arg("rclone")
            .output()
            .expect("Failed to run where command");
        let from = String::from_utf8(from.stdout).expect("Failed to convert output to string");
        let from = from.trim();
        let from = fs::canonicalize(from).expect("Failed to get canonical path");
        fs::copy(from, out_dir.join("rclone.exe")).unwrap();
    }
    #[cfg(target_os = "linux")]
    {
        fs::copy("/usr/bin/rclone", out_dir.join("rclone")).unwrap();
    }

    tauri_build::build()
}
