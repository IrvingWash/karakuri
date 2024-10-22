use kmath::Vector2;
use kutils::Color;

#[derive(Debug)]
pub struct Sprite {
    pub color: Color,
    pub size: Vector2,
}

impl Sprite {
    pub fn new(color: Color, size: Vector2) -> Self {
        Self { color, size }
    }
}
