use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

pub fn default() -> Menu {
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

    let file_menu = Submenu::new(
        "File",
        Menu::new()
            .add_item(CustomMenuItem::new("open", "Open...").accelerator("CmdOrCtrl+O"))
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::CloseWindow),
    );

    let window_menu = Submenu::new(
        "Window",
        Menu::new()
            .add_native_item(MenuItem::Minimize)
            .add_native_item(MenuItem::Zoom)
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::EnterFullScreen),
    );

    let help_menu = Submenu::new(
        "Help",
        Menu::new().add_item(CustomMenuItem::new("support", "Support URL...")),
    );

    #[cfg(target_os = "macos")]
    let menu = Menu::new()
        .add_submenu(app_menu)
        .add_submenu(file_menu)
        .add_submenu(window_menu)
        .add_submenu(help_menu);

    #[cfg(not(target_os = "macos"))]
    let menu = Menu::new()
        .add_submenu(file_menu)
        .add_submenu(window_menu)
        .add_submenu(help_menu);

    menu
}
