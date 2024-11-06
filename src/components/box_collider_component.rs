use kmath::Vector2;

#[derive(Debug)]
pub struct BoxColliderComponent {
    pub position_offset: Vector2,
    pub size: Option<Vector2>,
}

impl Default for BoxColliderComponent {
    fn default() -> Self {
        Self {
            position_offset: Vector2::ZERO,
            size: None,
        }
    }
}

impl BoxColliderComponent {
    #[inline]
    pub const fn new(position_offset: Vector2, size: Vector2) -> Self {
        Self {
            position_offset,
            size: Some(size),
        }
    }
}
