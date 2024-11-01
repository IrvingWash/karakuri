use karakuri::components::{ComponentPayload, SpriteComponent, TagComponent, TransformComponent};
use karakuri::utils::{Color, Size};
use karakuri::{Game, GameConfig};
use kmath::Vector2;

pub fn main() -> Result<(), String> {
    let mut game = Game::new(GameConfig {
        clear_color: Color::BLUE,
        resolution: Size::new(800, 600),
        target_fps: 60,
        title: "Sprites",
    });

    game.add_texture("sonic-idle", "./examples/sprite/assets/sprites/sonic.png")?;

    game.set_scene(vec![ComponentPayload {
        tag: Some(TagComponent::new(String::from("Sonic"))),
        transform: Some(TransformComponent::new(
            Vector2::new(100., 100.),
            Vector2::new(3., 2.),
            1.5708,
        )),
        sprite: Some(SpriteComponent {
            clip_position: None,
            clip_size: None,
            texture_name: "sonic-idle",
        }),
        figure: None,
        behavior: None,
    }]);

    game.start();

    Ok(())
}
