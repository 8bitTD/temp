# what is this?
This app displays the temperature outside

![test](https://github.com/8bitTD/temp/assets/19583059/587281ca-a255-4a95-a917-61f47c2f26ed)
## execution environment
* Windows10 64bit
## hot to use
* When you run it, something like the attached image will be displayed at the bottom right of the screen.

　![image](https://github.com/8bitTD/temp/assets/19583059/23ab2e72-53d9-4f99-9f6f-c3dde93414de)
* Move by left-dragging the mouse, exit by right-clicking.
* Please access the file below and enter your latitude, longitude and api_token.
```
C:\Users\(user_name)\Documents\script\Rust
 temp.json
```
![image](https://github.com/8bitTD/temp/assets/19583059/6bdb7d04-bb0e-4d6a-bc54-69d7d4dbe7a9)
* Please access the following site to obtain api_token.

>[https://home.openweathermap.org/users/sign_in](https://home.openweathermap.org/users/sign_in)

>[https://home.openweathermap.org/api_keys](https://home.openweathermap.org/api_keys)

## steps to execute
* Please install git in advance.
* Please install Rust in advance.
* Please enable Cargo to run tauri in advance.
```
git clone https://github.com/8bitTD/temp
cd temp
cargo tauri dev
cargo tauri build
run ./src-tauri/target/release/temp.exe
```
