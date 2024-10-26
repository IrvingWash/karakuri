use kutils::{Color, Size};

pub struct SpriteComponent {
    pub size: Size,
    pub color: Color,
}

impl SpriteComponent {
    pub fn new(size: Size, color: Color) -> Self {
        Self { size, color }
    }
}
