use kmath::Vector2;

#[derive(Debug)]
pub struct BoxColliderComponentParams {
    position_offset: Vector2,
    size: Option<Vector2>,
}

impl Default for BoxColliderComponentParams {
    #[inline]
    fn default() -> Self {
        Self {
            position_offset: Vector2::ZERO,
            size: None,
        }
    }
}

#[derive(Debug)]
pub struct BoxColliderComponent {
    pub position_offset: Vector2,
    pub size: Vector2,
}

impl BoxColliderComponent {
    #[inline]
    pub fn new(params: BoxColliderComponentParams, secondary_size: &Vector2) -> Self {
        let BoxColliderComponentParams {
            position_offset,
            size,
        } = params;

        Self {
            position_offset,
            size: size.unwrap_or(secondary_size.create_copy()),
        }
    }
}
