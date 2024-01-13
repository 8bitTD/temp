use serde::{Serialize, Deserialize};
use super::define::*;
#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherInfo{
    pub icon: String,
    pub weather: String,
    pub temp: String,
}
impl Default for WeatherInfo{
    fn default() -> WeatherInfo{
        WeatherInfo{
            icon: format!("{}{}", common::ICONURL,"50d.png"),
            weather: "-- ? --".into(),
            temp: "-- ? --".into(),
        }
    }
}
