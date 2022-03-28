use json_gettext::{get_text, static_json_gettext_build};
use sys_locale::get_locale;
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

pub fn default() -> Menu {
    let locale = get_locale()
        .unwrap_or_else(|| String::from("en-US"))
        .replace('_', "-");

    let ctx = static_json_gettext_build!(
        "en-US";
        "ar-DZ" => "locales/ar.json",
        "ar-EG" => "locales/ar.json",
        "ar-IQ" => "locales/ar.json",
        "ar-MA" => "locales/ar.json",
        "cs-CZ" => "locales/cs.json",
        "de-AT" => "locales/de.json",
        "de-CH" => "locales/de.json",
        "de-DE" => "locales/de.json",
        "en-GB" => "locales/en.json",
        "en-US" => "locales/en.json",
        "es-AR" => "locales/es.json",
        "es-CO" => "locales/es.json",
        "es-ES" => "locales/es.json",
        "es-MX" => "locales/es.json",
        "es-PE" => "locales/es.json",
        "es-US" => "locales/es.json",
        "ja-JP" => "locales/ja.json",
        "pl-PL" => "locales/pl.json",
        "pt-BR" => "locales/pt.json",
        "pt-PT" => "locales/pt.json",
        "ru-RU" => "locales/ru.json",
        "zh-CN" => "locales/zh_CN.json",
        "zh-TW" => "locales/zh_TW.json"
    )
    .unwrap();

    #[cfg(target_os = "macos")]
    let app_ctx = tauri::generate_context!();

    #[cfg(target_os = "macos")]
    let app_menu = Submenu::new(
        &app_ctx.package_info().name,
        Menu::new()
            .add_native_item(MenuItem::About(app_ctx.package_info().name.clone()))
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

    #[cfg(target_os = "macos")]
    let window_menu = Submenu::new(
        get_text!(ctx, &locale, "Window").unwrap().to_string(),
        Menu::new()
            .add_item(
                CustomMenuItem::new(
                    "minimize",
                    get_text!(ctx, &locale, "Minimize").unwrap().to_string(),
                )
                .accelerator("Cmd+M"),
            )
            .add_item(CustomMenuItem::new(
                "zoom",
                get_text!(ctx, &locale, "Zoom").unwrap().to_string(),
            ))
            .add_native_item(MenuItem::Separator)
            .add_item(
                CustomMenuItem::new(
                    "fullscreen",
                    get_text!(ctx, &locale, "Toggle Full Screen")
                        .unwrap()
                        .to_string(),
                )
                .accelerator("Cmd+Ctrl+F"),
            ),
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
                    get_text!(ctx, &locale, "Toggle Full Screen")
                        .unwrap()
                        .to_string(),
                )
                .accelerator("F11"),
            ),
    );

    #[cfg(target_os = "macos")]
    let menu = Menu::new()
        .add_submenu(app_menu)
        .add_submenu(file_menu)
        .add_submenu(window_menu);

    #[cfg(not(target_os = "macos"))]
    let menu = Menu::new().add_submenu(file_menu).add_submenu(window_menu);

    menu
}
