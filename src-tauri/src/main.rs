#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::path::{Path, PathBuf};
use tauri::api::dir::{read_dir, DiskEntry};
use tauri::api::shell;
use tauri::Manager;

mod menu;

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
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

fn is_dir(entry: &DiskEntry) -> bool {
    entry.children.is_some()
}

fn is_dot(entry: &DiskEntry) -> bool {
    match &entry.name {
        Some(name) => name.starts_with('.'),
        None => true,
    }
}

fn is_img(entry: &DiskEntry) -> bool {
    mime_from(&entry.path)
}

#[tauri::command]
async fn get_entries(dir: String) -> Vec<PathBuf> {
    let entries = match read_dir(dir, false) {
        Err(why) => {
            println!("Error in read_dir(): {:?}", why);
            let vec: Vec<DiskEntry> = Vec::new();
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
        .on_menu_event(|event| {
            if event.menu_item_id() == "support" {
                shell::open(
                    &event.window().shell_scope(),
                    "https://github.com/sprout2000/leafview2#readme",
                    None,
                )
                .unwrap();
            }
        })
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            #[cfg(debug_assertions)]
            window.open_devtools();

            let window_ = window.clone();
            window.on_menu_event(move |event| match event.menu_item_id() {
                "open" => window_
                    .emit(
                        "open",
                        Payload {
                            message: "open".into(),
                        },
                    )
                    .expect("Error while emitting open event"),
                "close" => std::process::exit(0),
                "minimize" => window_.minimize().unwrap(),
                "zoom" => {
                    if let Ok(result) = window_.is_maximized() {
                        if result {
                            window_.unmaximize().unwrap();
                        } else {
                            window_.maximize().unwrap();
                        }
                    }
                }
                "fullscreen" => {
                    if let Ok(result) = window_.is_fullscreen() {
                        if result {
                            window_.set_fullscreen(false).unwrap();
                        } else {
                            window_.set_fullscreen(true).unwrap();
                        }
                    }
                }
                _ => {}
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            mime_check,
            get_entries,
            move_to_trash,
        ])
        .plugin(tauri_plugin_window_state::WindowState::default())
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}
