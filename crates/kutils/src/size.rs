use kmath::Vector2;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Size {
    pub width: i64,
    pub height: i64,
}

impl Size {
    pub fn new(width: i64, height: i64) -> Self {
        Self { width, height }
    }

    pub fn to_scaled(&self, vector2: &Vector2) -> Size {
        Size {
            width: self.width * vector2.x as i64,
            height: self.height * vector2.y as i64,
        }
    }
}
