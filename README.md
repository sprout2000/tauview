# <img width="48" alt="leaves" src="https://user-images.githubusercontent.com/52094761/156916014-b9472d73-3270-455a-be95-25e527efeaff.svg" /> LeafView2

[![GitHub CI](https://github.com/sprout2000/leafview/actions/workflows/release.yml/badge.svg)](https://github.com/sprout2000/leafview/actions/workflows/release.yml)
[![GitHub stars](https://img.shields.io/github/stars/sprout2000/leafview2)](https://github.com/sprout2000/leafview2/stargazers)
![GitHub package.json dynamic](https://img.shields.io/github/package-json/keywords/sprout2000/leafview2)
[![GitHub license](https://img.shields.io/github/license/sprout2000/leafview)](https://github.com/sprout2000/leafview/blob/main/LICENSE.md)

A minimalist image viewer based on [Leaflet.js](https://leafletjs.com/) and [Tauri](https://tauri.studio/).

<img width="640" src="https://user-images.githubusercontent.com/52094761/157586637-4b2deb8e-a1f7-46ef-9f24-d0efeb6a97a8.png">

_Image by <a href="https://pixabay.com/users/myriams-fotos-1627417/?utm_source=link-attribution&amp;utm_medium=referral&amp;utm_campaign=image&amp;utm_content=1568646">Myriams-Fotos</a> from <a href="https://pixabay.com/?utm_source=link-attribution&amp;utm_medium=referral&amp;utm_campaign=image&amp;utm_content=1568646">Pixabay.</a>_

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

## :hammer_and_wrench: Development

```sh
% git clone https://github.com/sprout2000/leafview2.git
% cd leafview2
% yarn install

# for development
% yarn tauri dev

# for release
% yarn tauri build
```

_NOTE: You will need to meet [Tauri prerequisites](https://tauri.studio/docs/getting-started/prerequisites)._

## :copyright: License

Copyright (c) 2022 sprout2000 and other contributors
