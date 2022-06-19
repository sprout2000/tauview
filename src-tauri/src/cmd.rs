use std::path::{Path, PathBuf};
use tauri::api::dialog;
use tauri::api::dir;

fn mime_from<P: AsRef<Path>>(filepath: P) -> bool {
    match infer::get_from_path(filepath) {
        Ok(opt) => match opt {
            Some(filetype) => {
                filetype.mime_type() == "image/jpeg"
                    || filetype.mime_type() == "image/png"
                    || filetype.mime_type() == "image/gif"
                    || filetype.mime_type() == "image/webp"
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

fn is_dir(entry: &dir::DiskEntry) -> bool {
    entry.children.is_some()
}

fn is_dot(entry: &dir::DiskEntry) -> bool {
    match &entry.name {
        Some(name) => name.starts_with('.'),
        None => true,
    }
}

fn is_img(entry: &dir::DiskEntry) -> bool {
    mime_from(&entry.path)
}

#[tauri::command]
pub async fn open_dialog() -> Option<PathBuf> {
    dialog::blocking::FileDialogBuilder::new()
        .add_filter(
            "Image File",
            &[
                "ico", "ICO", "gif", "GIF", "png", "PNG", "jpg", "JPG", "jpeg", "JPEG", "webp",
                "WEBP",
            ],
        )
        .pick_file()
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
    let entries = match dir::read_dir(dir, false) {
        Err(why) => {
            println!("Error in read_dir(): {:?}", why);
            let vec: Vec<dir::DiskEntry> = Vec::new();
            vec
        }
        Ok(list) => list,
    };

    let mut paths = entries
        .iter()
        .filter(|entry| !is_dir(entry) && !is_dot(entry) && is_img(entry))
        .map(|entry| entry.path.to_path_buf())
        .collect::<Vec<PathBuf>>();

    paths.sort();
    paths
}
