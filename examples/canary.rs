use karakuri::{
    components::{SpriteComponent, TransformComponent},
    Game, GameConfig,
};
use kmath::Vector2;
use kutils::{Color, Size};

pub fn main() {
    let mut game = Game::new(GameConfig {
        title: "Sonic",
        resolution: Size::new(800, 600),
        target_fps: 60,
        min_update_fps: 30,
        clear_color: Color::BLUE,
    });

    let registry = game.registry();

    let knuckles = registry.create_entity();
    registry.add_component(
        &knuckles,
        TransformComponent::from_position(Vector2::new(300., 500.)),
    );
    registry.add_component(
        &knuckles,
        SpriteComponent::new(Size::new(300, 300), Color::RED),
    );

    let trigger = registry.create_entity();
    registry.add_component(&trigger, TransformComponent::default());

    let tails = registry.create_entity();
    registry.add_component(
        &tails,
        TransformComponent::from_position(Vector2::new(500., 300.)),
    );
    registry.add_component(
        &tails,
        SpriteComponent::new(Size::new(300, 300), Color::YELLOW),
    );

    game.start();
}
