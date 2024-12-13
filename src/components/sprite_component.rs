use kmath::Vector2;
use kutils::Color;

#[derive(Debug)]
pub struct SpriteComponentParams {
    pub texture_name: &'static str,
    pub clip_position: Vector2,
    pub clip_size: Option<Vector2>,
    pub tint: Color,
    pub origin: Option<Vector2>,
    pub layer: u8,
}

impl Default for SpriteComponentParams {
    #[inline]
    fn default() -> Self {
        Self {
            texture_name: "",
            clip_position: Vector2::ZERO,
            clip_size: None,
            tint: Color::WHITE,
            origin: None,
            layer: 0,
        }
    }
}

#[derive(Debug)]
pub struct SpriteComponent {
    pub texture_name: &'static str,
    pub clip_position: Vector2,
    pub clip_size: Vector2,
    pub tint: Color,
    pub origin: Vector2,
    pub layer: u8,
}

impl SpriteComponent {
    #[inline]
    pub fn new(
        params: SpriteComponentParams,
        secondary_clip_size: Vector2,
        secondary_origin: Vector2,
    ) -> Self {
        let SpriteComponentParams {
            clip_position,
            clip_size,
            layer,
            origin,
            texture_name,
            tint,
        } = params;

        Self {
            clip_position,
            clip_size: clip_size.unwrap_or(secondary_clip_size),
            layer,
            origin: origin.unwrap_or(secondary_origin),
            texture_name,
            tint,
        }
    }
}
