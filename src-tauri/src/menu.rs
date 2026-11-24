use json_gettext::{get_text, static_json_gettext_build};
use std::env::consts;
use sys_locale::get_locale;
use tauri::menu::{Menu, MenuBuilder, MenuItemBuilder, SubmenuBuilder};
use tauri::{AppHandle, Runtime};

fn get_close_accelerator() -> String {
    if consts::OS == "macos" {
        "Cmd+W".to_string()
    } else {
        "Alt+F4".to_string()
    }
}

fn get_fullscreen_accelerator() -> String {
    if consts::OS == "macos" {
        "Cmd+Ctrl+F".to_string()
    } else {
        "F11".to_string()
    }
}

#[cfg(debug_assertions)]
fn get_devtools_accelerator() -> String {
    if consts::OS == "macos" {
        "Alt+Cmd+I".to_string()
    } else {
        "Ctrl+Shift+I".to_string()
    }
}

pub fn default<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<Menu<R>> {
    let locale = get_locale().unwrap_or_else(|| String::from("en-US"));
    // https://source.chromium.org/chromium/chromium/src/+/main:ui/base/l10n/l10n_util.cc
    let ctx = static_json_gettext_build!(
        "en-US";
        "ar-AE" => "locales/ar.json",
        "ar-BH" => "locales/ar.json",
        "ar-DZ" => "locales/ar.json",
        "ar-EG" => "locales/ar.json",
        "ar-KW" => "locales/ar.json",
        "ar-IQ" => "locales/ar.json",
        "ar-JO" => "locales/ar.json",
        "ar-LB" => "locales/ar.json",
        "ar-MA" => "locales/ar.json",
        "ar-OM" => "locales/ar.json",
        "ar-QA" => "locales/ar.json",
        "ar-SA" => "locales/ar.json",
        "ar-TN" => "locales/ar.json",
        "ar-YE" => "locales/ar.json",
        "cs-CZ" => "locales/cs.json",
        "de-AT" => "locales/de.json",
        "de-CH" => "locales/de.json",
        "de-DE" => "locales/de.json",
        "de-LI" => "locales/de.json",
        "de-LU" => "locales/de.json",
        "en-GB" => "locales/en.json",
        "en-US" => "locales/en.json",
        "es-AR" => "locales/es.json",
        "es-BO" => "locales/es.json",
        "es-CL" => "locales/es.json",
        "es-CO" => "locales/es.json",
        "es-CR" => "locales/es.json",
        "es-DO" => "locales/es.json",
        "es-EC" => "locales/es.json",
        "es-ES" => "locales/es.json",
        "es-GT" => "locales/es.json",
        "es-HN" => "locales/es.json",
        "es-MX" => "locales/es.json",
        "es-NI" => "locales/es.json",
        "es-PA" => "locales/es.json",
        "es-PE" => "locales/es.json",
        "es-PR" => "locales/es.json",
        "es-PY" => "locales/es.json",
        "es-SV" => "locales/es.json",
        "es-US" => "locales/es.json",
        "es-UY" => "locales/es.json",
        "es-VE" => "locales/es.json",
        "fr-BE" => "locales/fr.json",
        "fr-CA" => "locales/fr.json",
        "fr-CH" => "locales/fr.json",
        "fr-FR" => "locales/fr.json",
        "fr-LU" => "locales/fr.json",
        "fr-MC" => "locales/fr.json",
        "hu-HU" => "locales/hu.json",
        "it" => "locales/it.json",
        "it-CH" => "locales/it.json",
        "it-IT" => "locales/it.json",
        "ja-JP" => "locales/ja.json",
        "pl-PL" => "locales/pl.json",
        "pt-BR" => "locales/pt.json",
        "pt-PT" => "locales/pt.json",
        "ru-RU" => "locales/ru.json",
        "tr" => "locales/tr.json",
        "uk" => "locales/uk.json",
        "zh-CN" => "locales/zh_CN.json",
        "zh-TW" => "locales/zh_TW.json"
    )
    .unwrap();

    let t = |key: &str| -> String {
        get_text!(ctx, locale.as_str(), key)
            .map(|v| v.to_string())
            .unwrap_or_else(|| key.to_string())
    };

    let file_menu = SubmenuBuilder::new(app, t("File"))
        .item(
            &MenuItemBuilder::with_id("open", t("Open..."))
                .accelerator("CmdOrCtrl+O")
                .build(app)?,
        )
        .separator()
        .item(
            &MenuItemBuilder::with_id("remove", t("Move to Trash"))
                .accelerator("Delete")
                .build(app)?,
        )
        .separator()
        .item(
            &MenuItemBuilder::with_id("close", t("Close"))
                .accelerator(get_close_accelerator())
                .build(app)?,
        )
        .build()?;

    let view_menu = SubmenuBuilder::new(app, t("View"))
        .item(
            &MenuItemBuilder::with_id("next", t("Next Image"))
                .accelerator("J")
                .build(app)?,
        )
        .item(
            &MenuItemBuilder::with_id("prev", t("Prev Image"))
                .accelerator("K")
                .build(app)?,
        )
        .separator()
        .item(
            &MenuItemBuilder::with_id("grid", t("Toggle Grid View"))
                .accelerator("H")
                .build(app)?,
        );

    // 開発用の項目は tauri dev（デバッグビルド）のときだけ表示する
    #[cfg(debug_assertions)]
    let view_menu = view_menu
        .separator()
        .item(
            &MenuItemBuilder::with_id("reload", t("Reload"))
                .accelerator("CmdOrCtrl+R")
                .build(app)?,
        )
        .item(
            &MenuItemBuilder::with_id("devtools", t("Toggle Developer Tools"))
                .accelerator(get_devtools_accelerator())
                .build(app)?,
        );

    let view_menu = view_menu.build()?;

    let window_menu = SubmenuBuilder::new(app, t("Window"))
        .item(
            &MenuItemBuilder::with_id("minimize", t("Minimize"))
                .accelerator("CmdOrCtrl+M")
                .build(app)?,
        )
        .item(&MenuItemBuilder::with_id("zoom", t("Zoom")).build(app)?)
        .separator()
        .item(
            &MenuItemBuilder::with_id("fullscreen", t("Toggle Fullscreen"))
                .accelerator(get_fullscreen_accelerator())
                .build(app)?,
        )
        .build()?;

    let help_menu = SubmenuBuilder::new(app, t("Help"))
        .item(&MenuItemBuilder::with_id("support", t("Support URL...")).build(app)?)
        .build()?;

    let menu = MenuBuilder::new(app);

    #[cfg(target_os = "macos")]
    let menu = {
        let app_menu = SubmenuBuilder::new(app, &app.package_info().name)
            .about(None)
            .separator()
            .hide()
            .hide_others()
            .show_all()
            .separator()
            .quit()
            .build()?;
        menu.item(&app_menu)
    };

    menu.item(&file_menu)
        .item(&view_menu)
        .item(&window_menu)
        .item(&help_menu)
        .build()
}
