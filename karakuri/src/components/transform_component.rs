use crate::math::Vector2;

#[derive(Debug, Clone)]
pub struct TransformComponent {
    pub position: Vector2,
    pub scale: Vector2,
    pub rotation: f64,
}

impl Default for TransformComponent {
    fn default() -> TransformComponent {
        TransformComponent::new(Vector2::zero(), Vector2::new(1., 1.), 0.)
    }
}

impl TransformComponent {
    pub fn new(position: Vector2, scale: Vector2, rotation: f64) -> TransformComponent {
        TransformComponent {
            position,
            scale,
            rotation,
        }
    }

    pub fn with_position(mut self, position: Vector2) -> TransformComponent {
        self.position = position;

        self
    }

    pub fn with_scale(mut self, scale: Vector2) -> TransformComponent {
        self.scale = scale;

        self
    }

    pub fn with_rotation(mut self, rotation: f64) -> TransformComponent {
        self.rotation = rotation;

        self
    }
}
