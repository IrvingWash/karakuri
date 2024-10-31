use kmath::Vector2;
use kutils::Size;

#[derive(Debug, Default)]
pub struct SpriteComponent {
    pub texture_name: &'static str,
    pub clip_position: Option<Vector2>,
    pub clip_size: Option<Size>,
}
