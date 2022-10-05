#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::env::consts;
use std::path::PathBuf;
use tauri::api::{dialog, shell};
use tauri::{Manager, Menu};

mod cmd;
mod menu;

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: Option<PathBuf>,
}

fn main() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .menu(if consts::OS == "macos" {
            menu::default(&context)
        } else {
            Menu::new()
        })
        .on_menu_event(|event| match event.menu_item_id() {
            "open" => dialog::FileDialogBuilder::new()
                .add_filter("Image File", &["ico", "gif", "png", "jpg", "jpeg", "webp"])
                .pick_file(move |f| {
                    event
                        .window()
                        .emit("open", Payload { message: f })
                        .expect("Error while emitting open event")
                }),
            "close" => std::process::exit(0),
            "minimize" => event.window().minimize().unwrap(),
            "zoom" => {
                if let Ok(result) = event.window().is_maximized() {
                    if result {
                        event.window().unmaximize().unwrap();
                    } else {
                        event.window().maximize().unwrap();
                    }
                }
            }
            "fullscreen" => {
                if let Ok(result) = event.window().is_fullscreen() {
                    if result {
                        event.window().set_fullscreen(false).unwrap();
                    } else {
                        event.window().set_fullscreen(true).unwrap();
                    }
                }
            }
            "support" => shell::open(
                &event.window().shell_scope(),
                "https://github.com/sprout2000/leafview-tauri#readme",
                None,
            )
            .expect("Error while opening external URL"),
            _ => {}
        })
        .setup(|app| {
            let _window = app.get_window("main").unwrap();
            #[cfg(debug_assertions)]
            _window.open_devtools();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            cmd::mime_check,
            cmd::open_dialog,
            cmd::get_entries,
            cmd::move_to_trash,
        ])
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .run(context)
        .expect("Error while running tauri application");
}
