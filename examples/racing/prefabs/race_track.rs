use karakuri::components::{ComponentPayload, SpriteComponent, TransformComponent};
use kmath::Vector2;

pub fn race_track_prefab(resolution: &Vector2) -> ComponentPayload {
    ComponentPayload {
        sprite: Some(SpriteComponent::from_texture_name("race_track")),
        transform: Some(TransformComponent::from_position(Vector2::new(
            resolution.x,
            resolution.y,
        ))),
        ..Default::default()
    }
}
