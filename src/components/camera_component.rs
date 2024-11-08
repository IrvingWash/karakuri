use kec::Entity;

#[derive(Debug, Default)]
pub struct CameraComponent {
    pub target: Option<Entity>,
}
