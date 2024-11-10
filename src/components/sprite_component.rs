use kmath::Vector2;
use kutils::Color;

#[derive(Debug, Clone)]
pub struct SpriteComponent {
    pub texture_name: &'static str,
    pub clip_position: Vector2,
    pub clip_size: Option<Vector2>,
    pub tint: Color,
    pub origin: Option<Vector2>,
    pub layer: u8,
}

impl Default for SpriteComponent {
    #[inline]
    fn default() -> Self {
        Self {
            texture_name: "",
            clip_position: Vector2::ZERO,
            clip_size: None,
            tint: Color::WHITE,
            layer: 0,
            origin: None,
        }
    }
}

impl SpriteComponent {
    #[inline]
    pub fn from_texture_name(texture_name: &'static str) -> Self {
        Self {
            texture_name,
            ..Default::default()
        }
    }
}
