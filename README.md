# LeafView (Tauri)

[![GitHub CI](https://github.com/sprout2000/leafview-tauri/actions/workflows/release.yml/badge.svg)](https://github.com/sprout2000/leafview-tauri/actions/workflows/release.yml)
[![GitHub all releases](https://img.shields.io/github/downloads/sprout2000/leafview-tauri/total)](https://github.com/sprout2000/leafview-tauri/releases)
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
|                       <kbd>Fn</kbd>+<kbd>Delete</kbd>                       | Move to Trash  |

### :computer_mouse: Mouse Operations

|    Mouse     | Function    |
| :----------: | :---------- |
|     Drag     | Pan         |
|    Wheel     | Zoom in/out |
| Double click | Reset zoom  |

## :globe_with_meridians: Supported Languages

| Language      | Code |     | Language  | Code |     | Language |  Code   |
| :------------ | :--: | :-: | :-------- | :--: | :-: | :------- | :-----: |
| اللغة العربية | `ar` |     | Français  | `fr` |     | Русский  |  `ru`   |
| Čeština       | `cs` |     | Magyar    | `hu` |     | 简体中文 | `zh_CN` |
| Deutsch       | `de` |     | 日本語    | `ja` |     | 繁体中文 | `zh_TW` |
| English       | `en` |     | Polski    | `pl` |     |
| Español       | `es` |     | Português | `pt` |     |

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

_NOTE: You will need to meet [Tauri prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites)_.

## :copyright: Copyright

Copyright (c) 2022 sprout2000 and other contributors
