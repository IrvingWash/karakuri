use karakuri::components::{
    Animation, AnimationControllerComponent, AnimationParams, ComponentPayload, SpriteComponent,
    TagComponent, TransformComponent,
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
        ..Default::default()
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
                clip_size: Some(Vector2::new(10.0, 10.0)),
                clip_position: Vector2::new(10.0, 10.0),
                ..Default::default()
            }),
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
            ..Default::default()
        },
        ComponentPayload {
            tag: Some(TagComponent::new(String::from("Radar"))),
            transform: Some(TransformComponent::from_position(Vector2::new(32.0, 32.0))),
            sprite: Some(SpriteComponent {
                texture_name: "radar",
                layer: 99,
                clip_size: Some(Vector2::new(64.0, 64.0)),
                ..Default::default()
            }),
            animation_controller: Some(AnimationControllerComponent::new(vec![
                (Animation::new(AnimationParams {
                    name: "idle",
                    texture_name: "radar",
                    frame_count: 8,
                    frame_rate: 10,
                    looping: true,
                })),
            ])),
            ..Default::default()
        },
    ]);

    game.start();

    Ok(())
}
