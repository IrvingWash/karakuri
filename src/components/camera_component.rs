use kec::Entity;

#[derive(Debug)]
pub struct CameraComponent {
    pub target: Option<Entity>,
    pub zoom: f64,
}

impl Default for CameraComponent {
    #[inline]
    fn default() -> Self {
        Self {
            target: None,
            zoom: 1.0,
        }
    }
}

#[cfg(test)]
mod camera_component_tests {
    use super::CameraComponent;

    #[test]
    fn test_default() {
        let camera = CameraComponent::default();

        assert_eq!(camera.target, None);
        assert_eq!(camera.zoom, 1.0);
    }
}
