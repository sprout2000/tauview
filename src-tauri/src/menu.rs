use json_gettext::get_text;
use sys_locale::get_locale;
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

mod locales;

pub fn default() -> Menu {
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
            .add_item(
                CustomMenuItem::new(
                    "close",
                    get_text!(ctx, &locale, "Close").unwrap().to_string(),
                )
                .accelerator("CmdOrCtrl+W"),
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
    let menu = Menu::new().add_submenu(file_menu).add_submenu(help_menu);

    menu
}
