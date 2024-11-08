use karakuri::components::{ComponentPayload, SpriteComponent, TransformComponent};
use kmath::Vector2;
use kutils::Size;

pub fn race_track_prefab(resolution: &Size) -> ComponentPayload {
    ComponentPayload {
        sprite: Some(SpriteComponent::from_texture_name("race_track")),
        transform: Some(TransformComponent::from_position(Vector2::new(
            resolution.width as f64,
            resolution.height as f64,
        ))),
        ..Default::default()
    }
}
