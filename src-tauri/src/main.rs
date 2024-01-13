#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod define;
mod window;
mod weather;

use serde::{Serialize, Deserialize};
use tauri::Manager;
use std::sync::Mutex;
use std::io::prelude::*;
use define::*;
use window::*;
use weather::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct MyApp{
    pub jsn: Jsn,
}
impl Default for MyApp{
    fn default() -> MyApp{
        MyApp{
            jsn: Jsn::default(),
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Jsn{
    window_info: WindowInfo,
    api_token: String,
    longitude: f32,
    lantitude: f32,
}
impl Default for Jsn{
    fn default() -> Jsn{
        Jsn{
            window_info: WindowInfo::default(),
            api_token: common::APITOKENDEFAULT.into(),
            lantitude: common::LANTITUDEDEFAULT,
            longitude: common::LONGITUDEDEFAULT,
        }
    }
}
impl Jsn{
    pub fn save_json(&self){
        let content = serde_json::to_string_pretty(&self).unwrap();
        let mut jsn_path: String = dirs::home_dir().unwrap().as_os_str().to_str().unwrap().to_string();
        let path = std::path::Path::new(&jsn_path);
        if !path.is_dir(){Some(std::fs::create_dir_all(path));}
        jsn_path.push_str(format!("{}{}{}",common::DOCUMENT,common::TOOLNAME,".json").as_str());
        let mut file = std::fs::File::create(&jsn_path).expect("create failed");
        file.write_all(content.as_bytes()).unwrap();
    }
    pub fn load_json(&mut self){
        let mut jsn_path: String = dirs::home_dir().unwrap().as_os_str().to_str().unwrap().to_string();
        let rust_path = format!("{}{}",&jsn_path,common::DOCUMENT);
        if !std::path::Path::new(&rust_path).is_dir(){Some(std::fs::create_dir_all(&rust_path));}
        jsn_path.push_str(format!("{}{}{}",common::DOCUMENT,common::TOOLNAME,".json").as_str());
        if std::fs::metadata(&jsn_path).is_err(){
            self.save_json();
            return;
        }
        let contents = std::fs::read_to_string(&jsn_path);
        if contents.is_ok(){
            let contents = contents.unwrap();
            let res:Result<Jsn,_> = serde_json::from_str(&contents);
            if res.is_ok(){*self = res.unwrap();}
        }
    }
}

#[tauri::command]
fn get_info(state: tauri::State<Mutex<MyApp>>) -> (f32, f32, String){
    let my_app = state.lock().unwrap();
    (my_app.jsn.lantitude, my_app.jsn.longitude, my_app.jsn.api_token.to_string())
}

#[tauri::command]
async fn get_weather(lantitude:f32, longitude: f32, apitoken: String ) -> WeatherInfo {
    let mut wi = WeatherInfo::default();
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&units=metric&lang=ja&appid={}",
        lantitude,
        longitude,
        apitoken,
    );
    let res: serde_json::Value = reqwest::Client::new().get(&url).send().await.unwrap().json().await.unwrap();
    //println!("{:?}", res);
    if res["main"] == serde_json::Value::Null{return wi;}
    let temp = &res["main"]["temp"].as_f64().unwrap();
    let weather = &res["weather"][0]["description"].as_str().unwrap().to_string();
    let icon = &res["weather"][0]["icon"].as_str().unwrap().to_string();
    wi.icon = format!("{}{}{}",common::ICONURL,icon,".png");
    wi.weather = weather.to_string();
    wi.temp = format!("{}℃",temp);
    wi
}

#[tauri::command]
fn move_window(app: tauri::AppHandle, state: tauri::State<Mutex<MyApp>>, sx: i32, sy: i32){
    let mut my_app = state.lock().unwrap();
    let wx = sx - 80;
    let wy = sy - 14;
    my_app.jsn.window_info.left = wx;
    my_app.jsn.window_info.top = wy;
    let pos = tauri::LogicalPosition{
        x: wx,
        y: wy,
    };
    let _ = app.get_window("main").unwrap().set_position(pos);
}

#[tauri::command]
fn save_window(state: tauri::State<Mutex<MyApp>>){
    let my_app = state.lock().unwrap();
    my_app.jsn.save_json();
}

#[tauri::command]
fn set_data(state: tauri::State<Mutex<MyApp>>, lantitude:f32, longitude: f32, apitoken: String){
    let mut my_app = state.lock().unwrap();
    my_app.jsn.lantitude = lantitude;
    my_app.jsn.longitude = longitude;
    my_app.jsn.api_token = apitoken;
    my_app.jsn.save_json();
    println!("{:?}", my_app);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_info,
            get_weather,
            move_window,
            save_window,
            set_data,
        ])
        .setup(|app|{
            let mut my_app = MyApp::default();
            my_app.jsn.load_json();
            let pos = tauri::LogicalPosition{
                x: my_app.jsn.window_info.left,
                y: my_app.jsn.window_info.top,
            };
            let size = tauri::LogicalSize{
                width: my_app.jsn.window_info.width, 
                height: my_app.jsn.window_info.height
            };
            let _ = app.get_window("main").unwrap().set_position(pos);
            let _ = app.get_window("main").unwrap().set_size(size);
            app.manage(Mutex::new(my_app));
            #[cfg(debug_assertions)]
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
