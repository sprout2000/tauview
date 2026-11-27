use json_gettext::{get_text, static_json_gettext_build};
use std::env::consts;
use sys_locale::get_locale;
use tauri::menu::{Menu, MenuBuilder, MenuItem, MenuItemBuilder, Submenu, SubmenuBuilder};
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

/// メニュー項目を作る。
///
/// Windows では WebView2 にフォーカスがあると、メニューのショートカットが（修飾キーを付けても）
/// メニューに届かない。そこで Windows ではキー入力をフロントエンド（App.tsx）で処理し、
/// メニュー項目にはショートカットを表示するだけにする（二重に実行されないように）。
/// macOS・Linux ではメニューにショートカットを登録する
fn item<R: Runtime>(
    app: &AppHandle<R>,
    id: &str,
    text: String,
    accelerator: Option<&str>,
) -> tauri::Result<MenuItem<R>> {
    #[cfg(target_os = "windows")]
    {
        let text = match accelerator {
            Some(accel) => format!("{text}\t{}", accel.replace("CmdOrCtrl", "Ctrl")),
            None => text,
        };
        MenuItemBuilder::with_id(id, text).build(app)
    }

    #[cfg(not(target_os = "windows"))]
    {
        let builder = MenuItemBuilder::with_id(id, text);
        match accelerator {
            Some(accel) => builder.accelerator(accel).build(app),
            None => builder.build(app),
        }
    }
}

/// OS の言語に合わせた翻訳関数を返す（訳がなければキーの英語をそのまま返す）
fn translator() -> impl Fn(&str) -> String {
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

    move |key: &str| -> String {
        get_text!(ctx, locale.as_str(), key)
            .map(|v| v.to_string())
            .unwrap_or_else(|| key.to_string())
    }
}

/// メニューバーのサブメニュー（ファイル・表示・ウィンドウ・ヘルプ）
fn submenus<R: Runtime>(
    app: &AppHandle<R>,
    t: &impl Fn(&str) -> String,
) -> tauri::Result<[Submenu<R>; 4]> {
    let file_menu = SubmenuBuilder::new(app, t("File"))
        .item(&item(app, "open", t("Open..."), Some("CmdOrCtrl+O"))?)
        .separator()
        .item(&item(app, "remove", t("Move to Trash"), Some("Delete"))?)
        .separator()
        .item(&item(
            app,
            "close",
            t("Close"),
            Some(get_close_accelerator().as_str()),
        )?)
        .build()?;

    let view_menu = SubmenuBuilder::new(app, t("View"))
        .item(&item(app, "next", t("Next Image"), Some("J"))?)
        .item(&item(app, "prev", t("Prev Image"), Some("K"))?)
        .separator()
        .item(&item(app, "grid", t("Toggle Grid View"), Some("H"))?);

    // 開発用の項目は tauri dev（デバッグビルド）のときだけ表示する
    #[cfg(debug_assertions)]
    let view_menu = view_menu
        .separator()
        .item(&item(app, "reload", t("Reload"), Some("CmdOrCtrl+R"))?)
        .item(&item(
            app,
            "devtools",
            t("Toggle Developer Tools"),
            Some(get_devtools_accelerator().as_str()),
        )?);

    let view_menu = view_menu.build()?;

    let window_menu = SubmenuBuilder::new(app, t("Window"))
        .item(&item(app, "minimize", t("Minimize"), Some("CmdOrCtrl+M"))?)
        .item(&item(app, "zoom", t("Zoom"), None)?)
        .separator()
        .item(&item(
            app,
            "fullscreen",
            t("Toggle Fullscreen"),
            Some(get_fullscreen_accelerator().as_str()),
        )?)
        .build()?;

    let help_menu = SubmenuBuilder::new(app, t("Help"))
        .item(&item(app, "support", t("Support URL..."), None)?)
        .build()?;

    Ok([file_menu, view_menu, window_menu, help_menu])
}

/// メニューバー（すべての OS で表示する。macOS ではアプリメニューも付ける）
pub fn default<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<Menu<R>> {
    let t = translator();
    let [file_menu, view_menu, window_menu, help_menu] = submenus(app, &t)?;

    let menu = MenuBuilder::new(app);

    #[cfg(target_os = "macos")]
    let menu = {
        let app_menu = SubmenuBuilder::new(app, &app.package_info().name)
            .about_with_text(t("About Tauview"), None)
            .separator()
            .hide_with_text(t("Hide Tauview"))
            .hide_others_with_text(t("Hide Others"))
            .show_all_with_text(t("Show All"))
            .separator()
            .quit_with_text(t("Quit Tauview"))
            .build()?;
        menu.item(&app_menu)
    };

    menu.item(&file_menu)
        .item(&view_menu)
        .item(&window_menu)
        .item(&help_menu)
        .build()
}
