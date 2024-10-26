use kutils::{Color, Size};

pub struct GameConfig {
    pub title: &'static str,
    pub resolution: Size,
    pub target_fps: u32,
    pub min_update_fps: u32,
    pub clear_color: Color,
}
