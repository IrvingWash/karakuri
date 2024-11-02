use karakuri::components::{
    AnimationComponent, ComponentPayload, SpriteComponent, TagComponent, TransformComponent,
};
use karakuri::math::Vector2;
use karakuri::utils::{Color, Size};
use karakuri::{Game, GameConfig};

pub fn main() -> Result<(), String> {
    let mut game = Game::new(&GameConfig {
        clear_color: Color::BLUE,
        resolution: Size::new(800, 600),
        target_fps: 60,
        title: "Sprites",
    });

    game.add_texture("sonic-idle", "./examples/sprite/assets/sprites/sonic.png")?;
    game.add_texture("radar", "./examples/sprite/assets/sprites/radar.png")?;

    game.set_scene(vec![
        ComponentPayload {
            tag: Some(TagComponent::new(String::from("Sonic"))),
            transform: Some(TransformComponent::new(
                Vector2::new(100., 100.),
                Vector2::new(1., 1.),
                45.0,
            )),
            sprite: Some(SpriteComponent {
                texture_name: "sonic-idle",
                layer: 1,
                clip_size: Some(Size::new(10, 10)),
                clip_position: Vector2::new(10.0, 10.0),
                ..Default::default()
            }),
            figure: None,
            behavior: None,
            ..Default::default()
        },
        ComponentPayload {
            tag: Some(TagComponent::new(String::from("Sonic Doppelganger"))),
            transform: Some(TransformComponent::new(
                Vector2::new(130., 130.),
                Vector2::new(5., 2.),
                45.0,
            )),
            sprite: Some(SpriteComponent {
                texture_name: "sonic-idle",
                layer: 0,
                tint: Color::YELLOW,
                ..Default::default()
            }),
            figure: None,
            behavior: None,
            ..Default::default()
        },
        ComponentPayload {
            tag: Some(TagComponent::new(String::from("Radar"))),
            transform: Some(TransformComponent::from_position(Vector2::new(
                300.0, 300.0,
            ))),
            sprite: Some(SpriteComponent {
                texture_name: "radar",
                layer: 99,
                clip_size: Some(Size::new(64, 64)),
                ..Default::default()
            }),
            animation: Some(AnimationComponent::new(8, 10, true)),
            ..Default::default()
        },
    ]);

    game.start();

    Ok(())
}
