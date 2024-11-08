use kec::Entity;
use kmath::Vector2;

#[derive(Debug, Default)]
pub struct CameraComponent {
    pub zoom: f64,
    pub offset: Vector2,
    pub target: Option<Entity>,
    pub rotation: f64,
}
