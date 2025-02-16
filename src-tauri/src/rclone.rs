use serde::Serialize;

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
