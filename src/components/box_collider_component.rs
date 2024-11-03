use kmath::Vector2;

#[derive(Debug)]
pub struct BoxColliderComponent {
    pub position_offset: Vector2,
    pub size_scale: Vector2,
}

impl Default for BoxColliderComponent {
    fn default() -> Self {
        Self {
            position_offset: Vector2::ZERO,
            size_scale: Vector2::new(1.0, 1.0),
        }
    }
}

impl BoxColliderComponent {
    pub fn new(position_offset: Vector2, size_scale: Vector2) -> Self {
        Self {
            position_offset,
            size_scale,
        }
    }
}
