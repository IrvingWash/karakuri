use karakuri::{
    components::{
        BehaviorComponent, ComponentPayload, SpriteComponent, TagComponent, TransformComponent,
    },
    Game, GameConfig,
};
use kmath::Vector2;
use kutils::{Color, Size};

#[derive(Debug)]
struct TailsScript {}

impl BehaviorComponent for TailsScript {
    fn on_start(&mut self) {}
    fn on_update(&mut self, delta_time: f64) {
        println!("{}", delta_time);
    }
    fn on_destroy(&mut self) {}
}

pub fn main() {
    let mut game = Game::new(GameConfig {
        title: "Sonic",
        resolution: Size::new(800, 600),
        target_fps: 60,
        min_update_fps: 30,
        clear_color: Color::BLUE,
    });

    game.set_scene(vec![
        ComponentPayload {
            tag: Some(TagComponent::new(String::from("Knuckles"))),
            transform: Some(TransformComponent::from_position(Vector2::new(300., 500.))),
            sprite: Some(SpriteComponent::new(Size::new(300, 300), Color::RED)),
            ..Default::default()
        },
        ComponentPayload {
            tag: Some(TagComponent::new(String::from("Trigger"))),
            transform: Some(TransformComponent::default()),
            ..Default::default()
        },
        ComponentPayload {
            tag: Some(TagComponent::new(String::from("Tails"))),
            transform: Some(TransformComponent::from_position(Vector2::new(500., 300.))),
            sprite: Some(SpriteComponent::new(Size::new(300, 300), Color::YELLOW)),
            behavior: Some(Box::new(TailsScript {})),
        },
    ]);

    game.start();
}
