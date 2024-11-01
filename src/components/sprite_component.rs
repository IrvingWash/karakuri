use kmath::Vector2;
use kutils::{Color, Size};

#[derive(Debug, Default)]
pub struct SpriteComponent {
    pub texture_name: &'static str,
    pub clip_position: Option<Vector2>,
    pub clip_size: Option<Size>,
    pub tint: Option<Color>,
    pub rotation_origin: Option<Vector2>,
}
