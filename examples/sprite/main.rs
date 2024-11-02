use karakuri::components::{ComponentPayload, SpriteComponent, TagComponent, TransformComponent};
use karakuri::math::Vector2;
use karakuri::utils::{Color, Size};
use karakuri::{Game, GameConfig};

pub fn main() -> Result<(), String> {
    let mut game = Game::new(GameConfig {
        clear_color: Color::BLUE,
        resolution: Size::new(800, 600),
        target_fps: 60,
        title: "Sprites",
    });

    game.add_texture("sonic-idle", "./examples/sprite/assets/sprites/sonic.png")?;

    game.set_scene(vec![
        ComponentPayload {
            tag: Some(TagComponent::new(String::from("Sonic"))),
            transform: Some(TransformComponent::new(
                Vector2::new(100., 100.),
                Vector2::new(3., 2.),
                45.0,
            )),
            sprite: Some(SpriteComponent {
                texture_name: "sonic-idle",
                clip_position: None,
                rotation_origin: None,
                clip_size: None,
                layer: 1,
                tint: Color::WHITE,
            }),
            figure: None,
            behavior: None,
        },
        ComponentPayload {
            tag: Some(TagComponent::new(String::from("Sonic Doppelganger"))),
            transform: Some(TransformComponent::new(
                Vector2::new(130., 130.),
                Vector2::new(3., 2.),
                45.0,
            )),
            sprite: Some(SpriteComponent {
                texture_name: "sonic-idle",
                clip_position: None,
                rotation_origin: None,
                clip_size: None,
                layer: 0,
                tint: Color::RED,
            }),
            figure: None,
            behavior: None,
        },
    ]);

    game.start();

    Ok(())
}
