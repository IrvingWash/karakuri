use crate::math::Vector2;

#[derive(Debug)]
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
}
