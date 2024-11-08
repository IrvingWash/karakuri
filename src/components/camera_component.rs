use kec::Entity;

#[derive(Debug)]
pub struct CameraComponent {
    pub target: Option<Entity>,
    pub zoom: f64,
}

impl Default for CameraComponent {
    fn default() -> Self {
        Self {
            target: None,
            zoom: 1.0,
        }
    }
}
