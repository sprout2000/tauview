#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::path::PathBuf;
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_opener::OpenerExt;

mod cmd;
mod format;
#[cfg(target_os = "macos")]
mod imageio;
#[cfg(not(target_os = "windows"))]
mod menu;
mod protocol;

use cmd::{OpenedState, Payload};

fn handle_menu_event(app: &AppHandle, id: &str) {
    match id {
        "open" => {
            let handle = app.clone();
            app.dialog()
                .file()
                .add_filter("Image File", format::EXTENSIONS)
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
        #[cfg(debug_assertions)]
        "reload" => {
            if let Some(window) = app.get_webview_window("main") {
                window
                    .eval("window.location.reload()")
                    .expect("Error while reloading the webview");
            }
        }
        #[cfg(debug_assertions)]
        "devtools" => {
            if let Some(window) = app.get_webview_window("main") {
                if window.is_devtools_open() {
                    window.close_devtools();
                } else {
                    window.open_devtools();
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
        // 2 つ目の起動を既存のウィンドウにまとめる（最初に登録する必要がある）
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.unminimize();
                let _ = window.set_focus();
            }

            let cwd = PathBuf::from(cwd);
            if let Some(path) = argv
                .iter()
                .skip(1)
                .map(|arg| cwd.join(arg))
                .find(|p| p.is_file())
            {
                cmd::open_path(app, path);
            }
        }))
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .manage(OpenedState::default())
        .register_asynchronous_uri_scheme_protocol("tauview", |_ctx, request, responder| {
            std::thread::spawn(move || responder.respond(protocol::handle(&request)));
        })
        .on_menu_event(|app, event| handle_menu_event(app, event.id().as_ref()))
        .setup(|app| {
            // Windows・Linux では関連付けたファイルがコマンドライン引数で渡される
            if let Some(path) = std::env::args_os()
                .skip(1)
                .map(PathBuf::from)
                .find(|p| p.is_file())
            {
                cmd::open_path(app.handle(), path);
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            cmd::frontend_ready,
            cmd::mime_check,
            cmd::open_dialog,
            cmd::get_entries,
            cmd::move_to_trash,
        ])
        .build(tauri::generate_context!())
        .expect("Error while building tauri application")
        .run(|_app, _event| {
            // macOS では関連付けたファイルが Apple Event で渡される（起動時・起動中とも）
            #[cfg(target_os = "macos")]
            if let tauri::RunEvent::Opened { urls } = _event {
                if let Some(path) = urls.into_iter().find_map(|url| url.to_file_path().ok()) {
                    cmd::open_path(_app, path);
                }
            }
        });
}
