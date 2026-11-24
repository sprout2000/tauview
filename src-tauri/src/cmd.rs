use std::env::consts;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, Manager, State};
use tauri_plugin_dialog::DialogExt;

use crate::format;

#[derive(Clone, serde::Serialize)]
pub struct Payload {
    pub message: Option<PathBuf>,
}

/// 関連付けやコマンドライン引数で渡されたファイルを、フロントエンドの準備ができるまで預かる
#[derive(Default)]
pub struct Opened {
    ready: bool,
    pending: Option<PathBuf>,
}

pub type OpenedState = Mutex<Opened>;

/// ファイルを開く。フロントエンドの準備ができていなければ預かっておく
pub fn open_path(app: &AppHandle, path: PathBuf) {
    let state = app.state::<OpenedState>();
    let mut opened = state.lock().unwrap();
    if opened.ready {
        app.emit(
            "open",
            Payload {
                message: Some(path),
            },
        )
        .expect("Error while emitting open event");
    } else {
        opened.pending = Some(path);
    }
}

fn is_dot(path: &Path) -> bool {
    match path.file_name().and_then(|name| name.to_str()) {
        Some(name) => {
            if consts::OS == "macos" {
                name.starts_with('.')
            } else {
                name.starts_with("._")
            }
        }
        None => true,
    }
}

/// フロントエンドの準備ができたことを受け取り、預かっていたファイルを返す
#[tauri::command]
pub fn frontend_ready(state: State<'_, OpenedState>) -> Option<PathBuf> {
    let mut opened = state.lock().unwrap();
    opened.ready = true;
    opened.pending.take()
}

#[tauri::command]
pub async fn open_dialog(app: AppHandle) -> Option<PathBuf> {
    app.dialog()
        .file()
        .add_filter("Image File", format::EXTENSIONS)
        .blocking_pick_file()
        .and_then(|f| f.into_path().ok())
}

#[tauri::command]
pub async fn mime_check(filepath: String) -> bool {
    format::detect(filepath).is_some()
}

#[tauri::command]
pub async fn move_to_trash(url: String) -> Result<(), String> {
    match trash::delete(url) {
        Ok(_) => Ok(()),
        Err(error) => Err(error.to_string()),
    }
}

#[tauri::command]
pub async fn get_entries(dir: String) -> Vec<PathBuf> {
    let entries = match fs::read_dir(dir) {
        Err(why) => {
            println!("Error in read_dir(): {:?}", why);
            return Vec::new();
        }
        Ok(list) => list,
    };

    let mut paths = entries
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|path| !path.is_dir() && !is_dot(path) && format::detect(path).is_some())
        .collect::<Vec<PathBuf>>();

    paths.sort();
    paths
}
