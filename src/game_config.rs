use kutils::{Color, Size};

#[derive(Debug)]
pub struct GameConfig {
    pub title: &'static str,
    pub resolution: Size,
    pub target_fps: u32,
    pub clear_color: Color,
    pub debug: bool,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            title: "karakuri",
            clear_color: Color::BLACK,
            debug: true,
            resolution: Size::new(800, 600),
            target_fps: 60,
        }
    }
}
