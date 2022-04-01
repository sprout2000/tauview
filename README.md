# <img width="48" alt="leaves" src="https://user-images.githubusercontent.com/52094761/156916014-b9472d73-3270-455a-be95-25e527efeaff.svg" /> LeafView2

[![GitHub CI](https://github.com/sprout2000/leafview/actions/workflows/release.yml/badge.svg)](https://github.com/sprout2000/leafview/actions/workflows/release.yml)
[![GitHub stars](https://img.shields.io/github/stars/sprout2000/leafview2)](https://github.com/sprout2000/leafview2/stargazers)
![GitHub package.json dynamic](https://img.shields.io/github/package-json/keywords/sprout2000/leafview2)
[![GitHub license](https://img.shields.io/github/license/sprout2000/leafview)](https://github.com/sprout2000/leafview/blob/main/LICENSE.md)

A minimalist image viewer for macOS based on [Leaflet.js](https://leafletjs.com/) and [Tauri](https://tauri.studio/).

<img width="640" src="https://user-images.githubusercontent.com/52094761/157586637-4b2deb8e-a1f7-46ef-9f24-d0efeb6a97a8.png">

_Image by <a href="https://pixabay.com/users/myriams-fotos-1627417/?utm_source=link-attribution&amp;utm_medium=referral&amp;utm_campaign=image&amp;utm_content=1568646">Myriams-Fotos</a> from <a href="https://pixabay.com/?utm_source=link-attribution&amp;utm_medium=referral&amp;utm_campaign=image&amp;utm_content=1568646">Pixabay</a>._

## :thumbsup: Features

- Pan & Wheel Zoom
- Browse the images in a folder

## :inbox_tray: Download

You can download the latest version of LeafView2 from the releases page here:
[https://github.com/sprout2000/leafview2/releases](https://github.com/sprout2000/leafview2/releases)

## :green_book: Usage

### :keyboard: Keyboard Shortcuts

|                                     Key                                     | Function       |
| :-------------------------------------------------------------------------: | :------------- |
|           <kbd>J</kbd> or <kbd>CmdOrCtrl</kbd>+<kbd>&#8594;</kbd>           | Next Image     |
|           <kbd>K</kbd> or <kbd>CmdOrCtrl</kbd>+<kbd>&#8592;</kbd>           | Previous Image |
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

| Language                               |  Code   |
| :------------------------------------- | :-----: |
| اللغة العربية                          |  `ar`   |
| Čeština :czech_republic:               |  `cs`   |
| Deutsch :de: :austria: :switzerland:   |  `de`   |
| English :us: :uk: :earth_americas:     |  `en`   |
| Español :es: :mexico: :argentina: :us: |  `es`   |
| 日本語 :jp:                            |  `ja`   |
| Polski :poland:                        |  `pl`   |
| Português :portugal: :brazil:          |  `pt`   |
| Русский :ru:                           |  `ru`   |
| 简体中文 :cn:                          | `zh_CN` |
| 繁体中文 :taiwan:                      | `zh_TW` |

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
  │   ├── en.json
+ │   └── ja.json
  ├── src
  └── tauri.conf.json
```

`src-tauri/ja.json`:

```json
{
  "About": "アバウト",
  "Hide": "隠す",
  "HideOthers": "その他を非表示にする",
  "Show All": "すべて表示",
  "Quit": "終了",
  "Minimize": "最小化",
  "Maximize": "最大化",
  "Zoom": "ズーム",
  "Toggle Full Screen": "フルスクリーンのトグル",
  "File": "ファイル",
  "Open...": "開く...",
  "Close": "閉じる",
  "Shortcuts": "ショートカット",
  "Next Image": "次の画像",
  "Prev Image": "前の画像",
  "Reset Zoom": "ズームをリセット",
  "Move to Trash": "ゴミ箱へ移動",
  "Window": "ウィンドウ"
}
```

2. Import the locale into [src-tauri/src/menu.rs](https://github.com/sprout2000/leafview2/blob/main/src-tauri/src/menu.rs) as follows:

```diff
      let ctx = static_json_gettext_build!(
          "en-US";
          "en-US" => "locales/en.json",
+         "ja-JP" => "locales/ja.json"
      )
      .unwrap();
```

3. And then please send a [pull request](https://github.com/sprout2000/leafview2/pulls) to this repository.

## :tada: Contributors

**Special Thanks to:**

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
% git clone https://github.com/sprout2000/leafview2.git
% cd leafview2
% npm install

# on development
% npx tauri dev

# on production
% npx tauri build
```

_NOTE: You will need to meet [Tauri prerequisites](https://tauri.studio/docs/getting-started/prerequisites)_.

## :copyright: License

Copyright (c) 2022 sprout2000 and other contributors
