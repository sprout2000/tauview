# LeafView (Tauri)

[![GitHub license](https://img.shields.io/github/license/sprout2000/leafview-tauri)](https://github.com/sprout2000/leafview-tauri/blob/main/LICENSE.md)
[![GitHub all releases](https://img.shields.io/github/downloads/sprout2000/leafview-tauri/total)](https://github.com/sprout2000/leafview-tauri/releases)
[![GitHub stars](https://img.shields.io/github/stars/sprout2000/leafview-tauri)](https://github.com/sprout2000/leafview-tauri/stargazers)

A minimalist image viewer for macOS based on [Leaflet.js](https://leafletjs.com/) and [Tauri](https://tauri.studio/).

<img width="640" alt="Image by PublicDomainPictures from Pixabay" src="https://user-images.githubusercontent.com/52094761/198956815-36e0affd-7fb5-4761-812c-794b83ffcd0f.png">

_Image by <a href="https://pixabay.com/ja/users/publicdomainpictures-14/?utm_source=link-attribution&amp;utm_medium=referral&amp;utm_campaign=image&amp;utm_content=163480">PublicDomainPictures</a> from <a href="https://pixabay.com/ja//?utm_source=link-attribution&amp;utm_medium=referral&amp;utm_campaign=image&amp;utm_content=163480">Pixabay</a>._

## :thumbsup: Features

- Pan & Wheel Zoom
- Browse the images in a folder
- Grid display mode is also available
- Available in [13 languages](#globe_with_meridians-supported-languages)

<img width="480" alt="Grid View mode" src="https://user-images.githubusercontent.com/52094761/198906109-6e477983-dc38-4858-9455-133016b99c65.png">

## :green_book: Usage

### :keyboard: Keyboard Shortcuts

|                         Key                         | Function         |
| :-------------------------------------------------: | :--------------- |
|                    <kbd>J</kbd>                     | Next Image       |
|                    <kbd>K</kbd>                     | Previous Image   |
|                    <kbd>H</kbd>                     | Toggle Grid View |
|                    <kbd>+</kbd>                     | Zoom In          |
|                    <kbd>-</kbd>                     | Zoom Out         |
|                    <kbd>0</kbd>                     | Reset Zoom       |
| <kbd>←</kbd> <kbd>↑</kbd> <kbd>↓</kbd> <kbd>→</kbd> | Pan              |
|  <kbd>Fn</kbd>+<kbd>Delete</kbd> or <kbd>Del</kbd>  | Move to Trash    |

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
