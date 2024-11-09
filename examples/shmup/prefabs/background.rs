use karakuri::components::{ComponentPayload, SpriteComponent, TransformComponent};
use kmath::Vector2;

pub fn background_prefab(
    resolution: &Vector2,
    texture_name: &'static str,
    layer: u8,
) -> ComponentPayload {
    ComponentPayload {
        transform: Some(TransformComponent {
            position: Vector2::new(resolution.x / 2.0, resolution.y / 2.0),
            scale: Vector2::new(2.0, 2.0),
            ..Default::default()
        }),
        sprite: Some(SpriteComponent {
            texture_name,
            layer,
            ..Default::default()
        }),
        ..Default::default()
    }
}
