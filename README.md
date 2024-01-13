# temp.exe
## 概要
このツールは、指定した位置の気温と天気を表示するツールです。

## 起動方法
```
git clone https://github.com/8bitTD/temp
cd temp
npm install
npm run tauri build
cd src-tauri/target/release
temp.exe
```
## 使い方
![image](https://github.com/8bitTD/temp/assets/19583059/66f3e2f8-d8db-4562-a761-8966575c2d42)

緯度、経度、[API](https://home.openweathermap.org/api_keys)を入力して決定ボタンを押してください。

![image](https://github.com/8bitTD/temp/assets/19583059/e7cef7a5-b2ef-49ef-94d1-73966c94c2b2)

* マウスの左ドラッグで位置を変更できます。
* マウスの右クリックで終了します。
* 緯度、経度、APIを編集したい場合は以下のファイルを編集してください
```
C:\Users\(ユーザー名)\Documents\script\Rust
 temp.json
```
## 動作確認
Windows 10
