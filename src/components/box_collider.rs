use kmath::Vector2;

#[derive(Debug)]
pub struct BoxCollider {
    pub position_offset: Vector2,
    pub size_scale: Vector2,
}

impl Default for BoxCollider {
    fn default() -> Self {
        Self {
            position_offset: Vector2::ZERO,
            size_scale: Vector2::new(1.0, 1.0),
        }
    }
}

impl BoxCollider {
    pub fn new(position_offset: Vector2, size_scale: Vector2) -> Self {
        Self {
            position_offset,
            size_scale,
        }
    }
}
