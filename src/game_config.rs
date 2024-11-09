use kmath::Vector2;
use kutils::Color;

#[derive(Debug)]
pub struct GameConfig {
    pub title: &'static str,
    pub resolution: Vector2,
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
            resolution: Vector2::new(800.0, 600.0),
            target_fps: 60,
        }
    }
}
