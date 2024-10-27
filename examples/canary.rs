use karakuri::{
    components::{
        BehaviorComponent, ComponentPayload, Ctx, SpriteComponent, TagComponent, TransformComponent,
    },
    Game, GameConfig,
};
use kmath::Vector2;
use kutils::{Color, Size};

#[derive(Debug)]
struct Tails {
    speed: f64,
}

impl BehaviorComponent for Tails {
    fn on_start(&mut self) {}

    fn on_update(&mut self, ctx: Ctx<'_>) {
        let mut velocity = Vector2::new(0., 0.);

        if ctx.input_result.w {
            velocity.y -= self.speed;
        }
        if ctx.input_result.a {
            velocity.x -= self.speed;
        }
        if ctx.input_result.s {
            velocity.y += self.speed;
        }
        if ctx.input_result.d {
            velocity.x += self.speed;
        }

        ctx.registry
            .get_component_mut::<TransformComponent>(ctx.entity)
            .unwrap()
            .position
            .add(&velocity.to_scaled(ctx.delta_time));
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
            behavior: Some(Box::new(Tails { speed: 100.0 })),
        },
    ]);

    game.start();
}
