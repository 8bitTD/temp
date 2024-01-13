use super::define::*;
#[derive(Debug,Clone,serde::Deserialize, serde::Serialize)] 
pub struct WindowInfo{
    pub left: i32,
    pub top: i32,
    pub width: u32,
    pub height: u32,
}
impl Default for WindowInfo{
    fn default() -> Self {
        Self {
            left: 0,
            top: 0,
            width: common::WINDOWWIDTH,
            height: common::WINDOWHEIGHT,
        }
    }
}