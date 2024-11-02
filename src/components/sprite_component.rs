use kmath::Vector2;
use kutils::{Color, Size};

#[derive(Debug)]
pub struct SpriteComponent {
    pub texture_name: &'static str,
    pub clip_position: Vector2,
    pub clip_size: Option<Size>,
    pub tint: Color,
    pub rotation_origin: Option<Vector2>,
    pub layer: u8,
}

impl Default for SpriteComponent {
    fn default() -> Self {
        Self {
            texture_name: "",
            clip_position: Vector2::ZERO,
            clip_size: None,
            tint: Color::WHITE,
            layer: 0,
            rotation_origin: None,
        }
    }
}

impl SpriteComponent {
    pub fn from_texture_name(texture_name: &'static str) -> Self {
        Self {
            texture_name,
            ..Default::default()
        }
    }
}
