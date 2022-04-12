#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use json_gettext::get_text;
use std::path::{Path, PathBuf};
use sys_locale::get_locale;
use tauri::api::dialog;
use tauri::api::dir::{read_dir, DiskEntry};
use tauri::api::shell;
use tauri::{CustomMenuItem, Manager, Menu, MenuItem, Submenu};

mod locales;

fn menu_items() -> Menu {
    let ctx = locales::default();
    let locale = get_locale()
        .unwrap_or_else(|| String::from("en-US"))
        .replace('_', "-");

    #[cfg(target_os = "macos")]
    let app_ctx = tauri::generate_context!();

    #[cfg(target_os = "macos")]
    let app_menu = Submenu::new(
        &app_ctx.package_info().name,
        Menu::new()
            .add_native_item(MenuItem::About(
                app_ctx.package_info().name.clone(),
                tauri::AboutMetadata::new(),
            ))
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Hide)
            .add_native_item(MenuItem::HideOthers)
            .add_native_item(MenuItem::ShowAll)
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Quit),
    );

    let close_menu = CustomMenuItem::new(
        "close",
        get_text!(ctx, &locale, "Close").unwrap().to_string(),
    )
    .accelerator("CmdOrCtrl+W");

    #[cfg(target_os = "macos")]
    let file_menu = Submenu::new(
        get_text!(ctx, &locale, "File").unwrap().to_string(),
        Menu::new().add_item(close_menu),
    );

    #[cfg(not(target_os = "macos"))]
    let file_menu = Submenu::new(
        get_text!(ctx, &locale, "File").unwrap().to_string(),
        Menu::new()
            .add_item(
                CustomMenuItem::new(
                    "open",
                    get_text!(ctx, &locale, "Open...").unwrap().to_string(),
                )
                .accelerator("CmdOrCtrl+O"),
            )
            .add_native_item(MenuItem::Separator)
            .add_item(close_menu),
    );

    #[cfg(not(target_os = "macos"))]
    let window_menu = Submenu::new(
        get_text!(ctx, &locale, "Window").unwrap().to_string(),
        Menu::new()
            .add_item(
                CustomMenuItem::new(
                    "minimize",
                    get_text!(ctx, &locale, "Minimize").unwrap().to_string(),
                )
                .accelerator("Ctrl+M"),
            )
            .add_item(CustomMenuItem::new(
                "zoom",
                get_text!(ctx, &locale, "Zoom").unwrap().to_string(),
            ))
            .add_native_item(MenuItem::Separator)
            .add_item(
                CustomMenuItem::new(
                    "fullscreen",
                    get_text!(ctx, &locale, "Toggle Fullscreen")
                        .unwrap()
                        .to_string(),
                )
                .accelerator("F11"),
            ),
    );

    let help_menu = Submenu::new(
        get_text!(ctx, &locale, "Help").unwrap().to_string(),
        Menu::new().add_item(CustomMenuItem::new(
            "support",
            get_text!(ctx, &locale, "Support URL...")
                .unwrap()
                .to_string(),
        )),
    );

    #[cfg(target_os = "macos")]
    let menu = Menu::new()
        .add_submenu(app_menu)
        .add_submenu(file_menu)
        .add_submenu(help_menu);

    #[cfg(not(target_os = "macos"))]
    let menu = Menu::new()
        .add_submenu(file_menu)
        .add_submenu(window_menu)
        .add_submenu(help_menu);

    menu
}

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
        .menu(menu_items())
        .on_menu_event(|event| {
            if event.menu_item_id() == "open" {
                dialog::FileDialogBuilder::new()
                    .add_filter("Image File", &["ico", "gif", "png", "jpg", "jpeg", "webp"])
                    .pick_file(move |f| {
                        event
                            .window()
                            .emit("open", Payload { message: f })
                            .expect("Error while emitting open event")
                    })
            }
        })
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            let window_ = window.clone();

            #[cfg(debug_assertions)]
            window.open_devtools();

            window.on_menu_event(move |event| match event.menu_item_id() {
                "minimize" => window_.minimize().unwrap(),
                "close" => std::process::exit(0),
                "zoom" => {
                    if let Ok(maximized) = window_.is_maximized() {
                        if maximized {
                            window_.unmaximize().unwrap();
                        } else {
                            window_.maximize().unwrap();
                        }
                    }
                }
                "fullscreen" => {
                    if let Ok(fullscreen) = window_.is_fullscreen() {
                        if fullscreen {
                            window_.set_fullscreen(false).unwrap();
                        } else {
                            window_.set_fullscreen(true).unwrap();
                        }
                    }
                }
                "support" => shell::open(
                    &window_.shell_scope(),
                    "https://github.com/sprout2000/leafview2#green_book-usage",
                    None,
                )
                .expect("Error while opening external URL"),
                _ => {}
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            mime_check,
            open_dialog,
            get_entries,
            move_to_trash,
        ])
        .plugin(tauri_plugin_window_state::WindowState::default())
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}
