# <img src="https://user-images.githubusercontent.com/52094761/193970617-a07eaee4-731e-4cc7-a553-f6333e01a46d.svg" height="48" /> LeafView (Tauri)

[![GitHub license](https://img.shields.io/github/license/sprout2000/leafview-tauri)](https://github.com/sprout2000/leafview-tauri/blob/main/LICENSE.md)
[![GitHub all releases](https://img.shields.io/github/downloads/sprout2000/leafview-tauri/total)](https://github.com/sprout2000/leafview-tauri/releases)
[![GitHub stars](https://img.shields.io/github/stars/sprout2000/leafview-tauri)](https://github.com/sprout2000/leafview-tauri/stargazers)

A minimalist image viewer based on [Leaflet.js](https://leafletjs.com/) and [Tauri](https://tauri.studio/).

<img width="640" alt="2022-10-05-115541" src="https://user-images.githubusercontent.com/52094761/193972873-4b562c06-09d7-46ff-b17f-b36e52677359.png">

_Image by <a href="https://pixabay.com/ja/users/publicdomainpictures-14/?utm_source=link-attribution&amp;utm_medium=referral&amp;utm_campaign=image&amp;utm_content=163480">PublicDomainPictures</a> from <a href="https://pixabay.com/ja//?utm_source=link-attribution&amp;utm_medium=referral&amp;utm_campaign=image&amp;utm_content=163480">Pixabay</a>._

## :thumbsup: Features

- Pan & Wheel Zoom
- Browse the images in a folder

## :inbox_tray: Download

### :computer: macOS

You can download the latest version of LeafView from the releases page here:
[https://github.com/sprout2000/leafview-tauri/releases](https://github.com/sprout2000/leafview-tauri/releases)

### :desktop_computer: Windows 10 & 11

You can get _(or upgrade to)_ the latest version of _LeafView_ via [winget](https://github.com/microsoft/winget-cli):

```sh
winget install sprout2000.LeafView-Tauri
```

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
|            <kbd>Fn</kbd>+<kbd>Delete</kbd> or <kbd>Delete</kbd>             | Move to Trash  |

### :computer_mouse: Mouse Operations

|    Mouse     | Function    |
| :----------: | :---------- |
|     Drag     | Pan         |
|    Wheel     | Zoom in/out |
| Double click | Reset zoom  |

## :hammer_and_wrench: Development

```sh
% git clone https://github.com/sprout2000/leafview-tauri.git
% cd leafview-tauri
% yarn install

# on development
% yarn tauri dev

# on production
% yarn tauri build
```

_NOTE: You will need to meet [Tauri prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites)_.

## :copyright: Copyright

Copyright (c) 2022 sprout2000 and other contributors
