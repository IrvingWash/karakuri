use kmath::Vector2;

#[derive(Debug, Clone)]
pub struct TransformComponent {
    pub position: Vector2,
    pub scale: Vector2,
    pub rotation: f64,
}

impl Default for TransformComponent {
    fn default() -> Self {
        Self::new(Vector2::ZERO, Vector2::new(1., 1.), 0.)
    }
}

impl TransformComponent {
    #[inline]
    pub fn new(position: Vector2, scale: Vector2, rotation: f64) -> Self {
        Self {
            position,
            scale,
            rotation,
        }
    }

    #[inline]
    pub fn from_position(position: Vector2) -> Self {
        Self {
            position,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod transform_tests {
    use kmath::Vector2;

    use super::TransformComponent;

    #[test]
    fn test_from_position() {
        let transform = TransformComponent::from_position(Vector2::new(10.25, 5.));

        assert_eq!(transform.position, Vector2::new(10.25, 5.));
        assert_eq!(transform.scale, Vector2::new(1., 1.));
        assert_eq!(transform.rotation, 0.);
    }
}
