use karakuri::components::{ComponentPayload, SpriteComponent, TransformComponent};
use kmath::Vector2;
use kutils::Size;

pub fn background_prefab(resolution: &Size, texture_name: &'static str) -> ComponentPayload {
    ComponentPayload {
        transform: Some(TransformComponent {
            position: Vector2::new(
                resolution.width as f64 / 2.0,
                resolution.height as f64 / 2.0,
            ),
            scale: Vector2::new(2.0, 2.0),
            ..Default::default()
        }),
        sprite: Some(SpriteComponent {
            texture_name,
            layer: 0,
            ..Default::default()
        }),
        ..Default::default()
    }
}
