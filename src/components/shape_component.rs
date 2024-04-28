use crate::{math::Vector2, utils::Color};

#[derive(Debug)]
pub struct ShapeComponent {
    pub color: Color,
    pub size: Vector2,
}

impl ShapeComponent {
    pub fn new(color: Color, size: Vector2) -> ShapeComponent {
        ShapeComponent { color, size }
    }
}
