use std::env::consts;
use std::fs;
use std::path::{Path, PathBuf};
use tauri::AppHandle;
use tauri_plugin_dialog::DialogExt;

pub const IMAGE_EXTENSIONS: [&str; 6] = ["ico", "gif", "png", "jpg", "jpeg", "webp"];

fn mime_from<P: AsRef<Path>>(filepath: P) -> bool {
    match infer::get_from_path(filepath) {
        Ok(opt) => match opt {
            Some(filetype) => {
                filetype.mime_type() == "image/jpeg"
                    || filetype.mime_type() == "image/png"
                    || filetype.mime_type() == "image/gif"
                    || filetype.mime_type() == "image/webp"
                    || filetype.mime_type() == "image/bmp"
                    || filetype.mime_type() == "image/vnd.microsoft.icon"
            }
            None => false,
        },
        Err(why) => {
            println!("Error in infer::get_from_path: {:?}", why);
            false
        }
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

#[tauri::command]
pub async fn open_dialog(app: AppHandle) -> Option<PathBuf> {
    app.dialog()
        .file()
        .add_filter("Image File", &IMAGE_EXTENSIONS)
        .blocking_pick_file()
        .and_then(|f| f.into_path().ok())
}

#[tauri::command]
pub async fn mime_check(filepath: String) -> bool {
    mime_from(filepath)
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
        .filter(|path| !path.is_dir() && !is_dot(path) && mime_from(path))
        .collect::<Vec<PathBuf>>();

    paths.sort();
    paths
}
