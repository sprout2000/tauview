#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::path::PathBuf;
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_opener::OpenerExt;

mod cmd;
#[cfg(not(target_os = "windows"))]
mod menu;

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: Option<PathBuf>,
}

fn handle_menu_event(app: &AppHandle, id: &str) {
    match id {
        "open" => {
            let handle = app.clone();
            app.dialog()
                .file()
                .add_filter("Image File", &cmd::IMAGE_EXTENSIONS)
                .pick_file(move |f| {
                    let message = f.and_then(|p| p.into_path().ok());
                    handle
                        .emit("open", Payload { message })
                        .expect("Error while emitting open event")
                });
        }
        "close" => app.exit(0),
        "next" => app
            .emit("menu-next", ())
            .expect("Error while emitting next event"),
        "prev" => app
            .emit("menu-prev", ())
            .expect("Error while emitting prev event"),
        "grid" => app
            .emit("menu-grid", ())
            .expect("Error while emitting grid event"),
        "remove" => app
            .emit("menu-remove", ())
            .expect("Error while emitting remove event"),
        "minimize" => {
            if let Some(window) = app.get_webview_window("main") {
                window.minimize().unwrap();
            }
        }
        "zoom" => {
            if let Some(window) = app.get_webview_window("main") {
                if let Ok(result) = window.is_maximized() {
                    if result {
                        window.unmaximize().unwrap();
                    } else {
                        window.maximize().unwrap();
                    }
                }
            }
        }
        "fullscreen" => {
            if let Some(window) = app.get_webview_window("main") {
                if let Ok(result) = window.is_fullscreen() {
                    window.set_fullscreen(!result).unwrap();
                }
            }
        }
        "support" => app
            .opener()
            .open_url("https://github.com/sprout2000/tauview#readme", None::<&str>)
            .expect("Error while opening external URL"),
        _ => {}
    }
}

fn main() {
    let builder = tauri::Builder::default();

    // Windows ではメニューバーを表示しない（v1 版と同じ挙動）
    #[cfg(not(target_os = "windows"))]
    let builder = builder.menu(|app| menu::default(app));

    builder
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .on_menu_event(|app, event| handle_menu_event(app, event.id().as_ref()))
        .setup(|app| {
            let _window = app.get_webview_window("main").unwrap();
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
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}
