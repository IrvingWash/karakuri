use kmath::Vector2;

pub struct Transform {
    pub position: Vector2,
    pub scale: Vector2,
    pub rotation: f64,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vector2::ZERO,
            scale: Vector2::new(1., 1.),
            rotation: 0.,
        }
    }
}

impl Transform {
    pub fn new(position: Vector2, scale: Vector2, rotation: f64) -> Self {
        Self {
            position,
            scale,
            rotation,
        }
    }

    pub fn from_position(position: Vector2) -> Self {
        Self {
            position,
            ..Default::default()
        }
    }
}
