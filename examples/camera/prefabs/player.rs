use std::cell::RefMut;

use karakuri::components::{
    BehaviorComponent, ComponentPayload, SpriteComponent, TagComponent, TransformComponent,
};
use karakuri::math::Vector2;
use karakuri::utils::Size;
use karakuri::InputProcessorAdapter;
use kwindow::KeyboardKey;

pub fn player_prefab(resolution: &Size) -> ComponentPayload {
    ComponentPayload {
        tag: Some(TagComponent::new(String::from("player"))),
        transform: Some(TransformComponent::from_position(Vector2::new(
            resolution.width as f64 / 2.0,
            resolution.height as f64 / 2.0,
        ))),
        sprite: Some(SpriteComponent::from_texture_name("player")),
        behavior: Some(Box::new(Player::new(50.0))),
        ..Default::default()
    }
}

#[derive(Debug)]
struct Player {
    speed: f64,
}

impl Player {
    fn new(speed: f64) -> Self {
        Self { speed }
    }

    fn movement_handler(
        &mut self,
        mut transform: RefMut<TransformComponent>,
        input_processor: &InputProcessorAdapter,
        delta_time: f64,
    ) {
        if input_processor.is_down(KeyboardKey::KEY_W) {
            transform.position.y -= self.speed * delta_time;
        }

        if input_processor.is_down(KeyboardKey::KEY_A) {
            transform.position.x -= self.speed * delta_time;
        }

        if input_processor.is_down(KeyboardKey::KEY_S) {
            transform.position.y += self.speed * delta_time;
        }

        if input_processor.is_down(KeyboardKey::KEY_D) {
            transform.position.x += self.speed * delta_time;
        }
    }
}

impl BehaviorComponent for Player {
    fn on_update(&mut self, ctx: karakuri::UpdateContext) {
        let delta_time = ctx.delta_time;

        let transform = ctx
            .registry
            .get_component_mut::<TransformComponent>(ctx.entity)
            .unwrap();

        self.movement_handler(transform, ctx.input_processor, delta_time);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
