use kmath::Vector2;
use kutils::{Color, Size};

#[derive(Debug)]
pub struct SpriteComponent {
    pub texture_name: &'static str,
    pub clip_position: Option<Vector2>,
    pub clip_size: Option<Size>,
    pub tint: Color,
    pub rotation_origin: Option<Vector2>,
    pub layer: u8,
}

impl SpriteComponent {
    pub fn from_texture_name(texture_name: &'static str) -> Self {
        Self {
            texture_name,
            clip_position: None,
            clip_size: None,
            tint: Color::WHITE,
            rotation_origin: None,
            layer: 0,
        }
    }
}
