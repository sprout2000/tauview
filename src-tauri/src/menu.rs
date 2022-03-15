use json_gettext::{get_text, static_json_gettext_build, JSONGetText};
use sys_locale::get_locale;
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

fn i18n(ctx: &JSONGetText, locale: &str, fallback: &str) -> String {
    let title = match get_text!(ctx, locale, fallback) {
        Some(result) => result.to_string(),
        None => fallback.to_string(),
    };
    title
}

pub fn default() -> Menu {
    let locale = get_locale().unwrap_or_else(|| String::from("en-US"));

    let ctx = static_json_gettext_build!(
        "en-US";
        "cs-CZ" => "locales/cs.json",
        "de-AT" => "locales/de.json",
        "de-CH" => "locales/de.json",
        "de-DE" => "locales/de.json",
        "en-GB" => "locales/en.json",
        "en-US" => "locales/en.json",
        "es-AR" => "locales/es.json",
        "es-ES" => "locales/es.json",
        "es-MX" => "locales/es.json",
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
        i18n(&ctx, &locale, "File"),
        Menu::new()
            .add_item(
                CustomMenuItem::new("open", i18n(&ctx, &locale, "Open..."))
                    .accelerator("CmdOrCtrl+O"),
            )
            .add_native_item(MenuItem::Separator)
            .add_item(
                CustomMenuItem::new("close", i18n(&ctx, &locale, "Close"))
                    .accelerator("CmdOrCtrl+W"),
            ),
    );

    #[cfg(target_os = "macos")]
    let shortcuts_menu = Submenu::new(
        i18n(&ctx, &locale, "Shortcuts"),
        Menu::new()
            .add_item(
                CustomMenuItem::new("next", i18n(&ctx, &locale, "Next Image")).accelerator("J"),
            )
            .add_item(
                CustomMenuItem::new("prev", i18n(&ctx, &locale, "Prev Image")).accelerator("K"),
            )
            .add_item(
                CustomMenuItem::new("reset", i18n(&ctx, &locale, "Reset Zoom")).accelerator("0"),
            )
            .add_native_item(MenuItem::Separator)
            .add_item(
                CustomMenuItem::new("trash", i18n(&ctx, &locale, "Move to Trash"))
                    .accelerator("Delete"),
            ),
    );

    #[cfg(not(target_os = "linux"))]
    let window_menu = Submenu::new(
        i18n(&ctx, &locale, "Window"),
        Menu::new()
            .add_native_item(MenuItem::Minimize)
            .add_native_item(MenuItem::Zoom),
    );

    #[cfg(target_os = "macos")]
    let menu = Menu::new()
        .add_submenu(app_menu)
        .add_submenu(file_menu)
        .add_submenu(window_menu)
        .add_submenu(shortcuts_menu);

    #[cfg(target_os = "windows")]
    let menu = Menu::new().add_submenu(file_menu).add_submenu(window_menu);

    #[cfg(target_os = "linux")]
    let menu = Menu::new().add_submenu(file_menu);

    menu
}
