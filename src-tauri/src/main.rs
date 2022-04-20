#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::path::{Path, PathBuf};
use tauri::api::{dialog, dir, shell};
use tauri::Manager;

mod menu;

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: Option<PathBuf>,
}

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

#[tauri::command]
async fn open_dialog() -> Option<PathBuf> {
    dialog::blocking::FileDialogBuilder::new()
        .add_filter("Image File", &["ico", "gif", "png", "jpg", "jpeg", "webp"])
        .pick_file()
}

#[tauri::command]
async fn mime_check(filepath: String) -> bool {
    mime_from(filepath)
}

#[tauri::command]
async fn move_to_trash(url: String) -> Result<(), String> {
    match trash::delete(url) {
        Ok(_) => Ok(()),
        Err(error) => Err(error.to_string()),
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
async fn get_entries(dir: String) -> Vec<PathBuf> {
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

fn main() {
    tauri::Builder::default()
        .menu(menu::default())
        .on_menu_event(|event| match event.menu_item_id() {
            "open" => dialog::FileDialogBuilder::new()
                .add_filter("Image File", &["ico", "gif", "png", "jpg", "jpeg", "webp"])
                .pick_file(move |f| {
                    event
                        .window()
                        .emit("open", Payload { message: f })
                        .expect("Error while emitting open event")
                }),
            "support" => shell::open(
                &event.window().shell_scope(),
                "https://github.com/sprout2000/leafview2",
                None,
            )
            .expect("Error while opening external URL"),
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            mime_check,
            open_dialog,
            get_entries,
            move_to_trash,
        ])
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}
