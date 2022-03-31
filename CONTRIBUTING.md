# :scroll: Adding Locales

You can contribute to this repository very easily!

1. Fork this repository.

<img width="480" alt="スクリーンショット 2022-02-02 17 01 54" src="https://user-images.githubusercontent.com/52094761/152115921-ec22558b-df83-43fa-b0cd-754fbb687988.png">

2. Create `{your_LANG}.json` in `src-tauri/locales`.

```diff
src-tauri
  ├── .gitignore
  ├── Cargo.lock
  ├── Cargo.toml
  ├── build.rs
  ├── icons
  ├── locales
  │   ├── ar.json
  │   ├── cs.json
  │   ├── de.json
  │   ├── en.json
  │   ├── es.json
+ │   ├── ja.json
  │   ├── pl.json
  │   ├── pt.json
  │   ├── ru.json
  │   ├── zh_CN.json
  │   └── zh_TW.json
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

3. Send the [pull request](https://github.com/sprout2000/leafview2/pulls) to this repo.
