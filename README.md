# ![vector](https://user-images.githubusercontent.com/52094761/168923064-f34b8574-ae0a-4458-8f6e-b0279777323d.png) LeafView (Tauri)

[![GitHub CI](https://github.com/sprout2000/leafview-tauri/actions/workflows/release.yml/badge.svg)](https://github.com/sprout2000/leafview-tauri/actions/workflows/release.yml)
[![GitHub stars](https://img.shields.io/github/stars/sprout2000/leafview-tauri)](https://github.com/sprout2000/leafview-tauri/stargazers)
[![GitHub license](https://img.shields.io/github/license/sprout2000/leafview-tauri)](https://github.com/sprout2000/leafview-tauri/blob/main/LICENSE.md)

A minimalist image viewer for macOS based on [Leaflet.js](https://leafletjs.com/) and [Tauri](https://tauri.studio/).

<img width="640" src="https://user-images.githubusercontent.com/52094761/157586637-4b2deb8e-a1f7-46ef-9f24-d0efeb6a97a8.png">

_Image by <a href="https://pixabay.com/users/myriams-fotos-1627417/?utm_source=link-attribution&amp;utm_medium=referral&amp;utm_campaign=image&amp;utm_content=1568646">Myriams-Fotos</a> from <a href="https://pixabay.com/?utm_source=link-attribution&amp;utm_medium=referral&amp;utm_campaign=image&amp;utm_content=1568646">Pixabay</a>._

## :thumbsup: Features

- Pan & Wheel Zoom
- Browse the images in a folder

## :inbox_tray: Download

You can download the latest version of LeafView from the releases page here:
[https://github.com/sprout2000/leafview-tauri/releases](https://github.com/sprout2000/leafview-tauri/releases)

## :green_book: Usage

### :keyboard: Keyboard Shortcuts

|                                     Key                                     | Function       |
| :-------------------------------------------------------------------------: | :------------- |
|               <kbd>J</kbd> or <kbd>⌘</kbd>+<kbd>&#8594;</kbd>               | Next Image     |
|               <kbd>K</kbd> or <kbd>⌘</kbd>+<kbd>&#8592;</kbd>               | Previous Image |
|                                <kbd>+</kbd>                                 | Zoom In        |
|                                <kbd>-</kbd>                                 | Zoom Out       |
|                                <kbd>0</kbd>                                 | Reset Zoom     |
| <kbd>&#8592;</kbd> <kbd>&#8593;</kbd> <kbd>&#8595;</kbd> <kbd>&#8594;</kbd> | Pan            |

### :computer_mouse: Mouse Operations

|    Mouse     | Function    |
| :----------: | :---------- |
|     Drag     | Pan         |
|    Wheel     | Zoom in/out |
| Double click | Reset zoom  |

## :globe_with_meridians: Supported Languages

| Language      |  Code   |
| :------------ | :-----: |
| اللغة العربية |  `ar`   |
| Čeština       |  `cs`   |
| Deutsch       |  `de`   |
| English       |  `en`   |
| Español       |  `es`   |
| Magyar nyelv  |  `hu`   |
| 日本語        |  `ja`   |
| Polski        |  `pl`   |
| Português     |  `pt`   |
| Русский       |  `ru`   |
| 简体中文      | `zh_CN` |
| 繁体中文      | `zh_TW` |

## :scroll: Contributing

You can easily contribute to this repository by providing translation files.

1. Create `{your_LANG}.json` in `src-tauri/locales`.

```diff
src-tauri
  ├── .gitignore
  ├── Cargo.lock
  ├── Cargo.toml
  ├── build.rs
  ├── icons
  ├── locales
+ │   ├── cs.json
  │   ├── en.json
  │   └── ja.json
  ├── src
  └── tauri.conf.json
```

`src-tauri/locales/cs.json`:

```json
{
  "File": "Soubor",
  "Open...": "Otevřít...",
  "Select an image": "Vybrat obrázek",
  "Image files": "Soubory obrázku",
  "Move to Trash": "Přesunout do koše",
  "View": "Zobrazit",
  "Next Image": "Následující obrázek",
  "Prev Image": "Předchozí obrázek",
  "Toggle Fullscreen": "Celá obrazovka",
  "Toggle Developer Tools": "Zobrazit nástroje pro vývojáře",
  "Toggle Menubar": "Přepnout lištu nabídek",
  "Toggle Dark Mode": "Přepínání tmavého režimu",
  "Window": "Okno",
  "Minimize": "Minimalizovat",
  "Maximize": "Maximalizovat",
  "Zoom": "Přiblížit",
  "Bring All to Front": "Přenést vše do popředí",
  "Close": "Storno",
  "Help": "Nápověda",
  "About": "O aplikaci LeafView",
  "About LeafView": "O aplikaci LeafView",
  "Support URL...": "URL podpory...",
  "Hide LeafView": "Skrýt LeafView",
  "Hide Others": "Skrýt ostatní",
  "Show All": "Zobrazit vše",
  "Quit": "Ukončit LeafView",
  "Quit LeafView": "Ukončit LeafView"
}
```

2. Import the locale into [src-tauri/src/locales.rs](https://github.com/sprout2000/leafview-tauri/blob/main/src-tauri/src/locales.rs) and [src/setLocales.ts](https://github.com/sprout2000/leafview-tauri/blob/main/src/setLocales.ts) as follows:

`src-tauri/src/locales.rs`:

```diff
      let locales = static_json_gettext_build!(
          "en-US";
          "en-US" => "locales/en.json",
          "ja-JP" => "locales/ja.json",
+         "cs-CZ" => "locales/cs.json"
      )
      .unwrap();
```

`src/setLocales.ts`:

```diff
  import en from '../src-tauri/locales/en.json';
  import ja from '../src-tauri/locales/ja.json';
+ import cs from '../src-tauri/locales/cs.json';

  export const setLocales = (locale: string): void => {
    i18next.init({
      lng: locale,
      fallbackLng: 'en',
      resources: {
        en: { translation: en },
        ja: { translation: ja },
+       cs: { translation: cs },
      },
    });
  };
```

3. And then please send a [pull request](https://github.com/sprout2000/leafview-tauri/pulls) to this repository.

## :tada: Contributors

**Special Thanks to:**

- [@Levminer](https://github.com/Levminer) [#305](https://github.com/sprout2000/leafview/pull/305)
- [@SuhaibAtef](https://github.com/SuhaibAtef) [#274](https://github.com/sprout2000/leafview/pull/274)
- [@mwoz123](https://github.com/mwoz123) [#260](https://github.com/sprout2000/leafview/pull/260), [#261](https://github.com/sprout2000/leafview/pull/261)
- [@ArcherGu](https://github.com/ArcherGu) [#235](https://github.com/sprout2000/leafview/pull/235)
- [@guaycuru](https://github.com/guaycuru) [#228](https://github.com/sprout2000/leafview/pull/228), [#232](https://github.com/sprout2000/leafview/pull/232)
- [@kitt3911](https://github.com/kitt3911) [#215](https://github.com/sprout2000/leafview/pull/215)
- [@nukeop](https://github.com/nukeop) [#214](https://github.com/sprout2000/leafview/pull/214)
- [@singuerinc](https://github.com/singuerinc) [#178](https://github.com/sprout2000/leafview/pull/178)
- [@DrDeee](https://github.com/DrDeee) [#166](https://github.com/sprout2000/leafview/pull/166)
- [@PetrTodorov](https://github.com/PetrTodorov) [#68](https://github.com/sprout2000/leafview/pull/68)

## :hammer_and_wrench: Development

```sh
% git clone https://github.com/sprout2000/leafview-tauri.git
% cd leafview-tauri
% npm install

# on development
% npx tauri dev

# on production
% npx tauri build
```

_NOTE: You will need to meet [Tauri prerequisites](https://tauri.studio/docs/getting-started/prerequisites)_.

## :copyright: License

Copyright (c) 2022 sprout2000 and other contributors
