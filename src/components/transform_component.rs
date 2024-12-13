use kmath::Vector2;

#[derive(Debug)]
pub struct TransformComponentParams {
    pub position: Vector2,
    pub scale: Vector2,
    pub rotation: f64,
}

impl Default for TransformComponentParams {
    #[inline]
    fn default() -> Self {
        Self {
            position: Vector2::ZERO,
            scale: Vector2::new(1.0, 1.0),
            rotation: 0.0,
        }
    }
}

#[derive(Debug)]
pub struct TransformComponent {
    pub position: Vector2,
    pub scale: Vector2,
    pub rotation: f64,
}

impl TransformComponent {
    #[inline]
    pub const fn new(params: TransformComponentParams) -> Self {
        let TransformComponentParams {
            position,
            scale,
            rotation,
        } = params;

        Self {
            position,
            scale,
            rotation,
        }
    }
}
