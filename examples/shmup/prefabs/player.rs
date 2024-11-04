use std::cell::RefMut;

use karakuri::components::{
    BehaviorComponent, BoxColliderComponent, ComponentPayload, Ctx, RigidBodyComponent,
    SpriteComponent, TagComponent, TransformComponent,
};
use karakuri::ec::Entity;
use karakuri::math::Vector2;
use karakuri::utils::Size;
use karakuri::window::KeyboardKey;

use super::player_laser::player_laser_prefab;

pub fn player_prefab(resolution: &Size) -> ComponentPayload {
    ComponentPayload {
        transform: Some(TransformComponent::default()),
        box_collider: Some(BoxColliderComponent::default()),
        rigid_body: Some(RigidBodyComponent::default()),
        tag: Some(TagComponent::new(String::from("player"))),
        sprite: Some(SpriteComponent::from_texture_name("ship_blue")),
        behavior: Some(Box::new(Player::new(100.0, resolution.clone()))),
        ..Default::default()
    }
}

#[derive(Debug)]
struct Player {
    speed: f64,
    resolution: Size,
}

impl Player {
    fn new(speed: f64, resolution: Size) -> Self {
        Self { speed, resolution }
    }

    fn movement_handler(&mut self, ctx: &Ctx, rigid_body: &mut RefMut<RigidBodyComponent>) {
        let speed = if ctx.input_processor.is_down(KeyboardKey::KEY_LEFT_SHIFT) {
            self.speed * 2.0
        } else {
            self.speed
        };

        if ctx.input_processor.is_down(KeyboardKey::KEY_W) {
            rigid_body.velocity.y = -speed * ctx.delta_time;
        }
        if ctx.input_processor.is_down(KeyboardKey::KEY_A) {
            rigid_body.velocity.x = -speed * ctx.delta_time;
        }
        if ctx.input_processor.is_down(KeyboardKey::KEY_S) {
            rigid_body.velocity.y = speed * ctx.delta_time;
        }
        if ctx.input_processor.is_down(KeyboardKey::KEY_D) {
            rigid_body.velocity.x = speed * ctx.delta_time;
        }
    }

    fn fire(&self, ctx: &mut Ctx, transform: TransformComponent) {
        if ctx.input_processor.is_pressed(KeyboardKey::KEY_SPACE) {
            ctx.spawner.add_entity(player_laser_prefab(transform));
        }
    }
}

impl BehaviorComponent for Player {
    fn on_start(&mut self, ctx: karakuri::components::Ctx) {
        let mut transform = ctx
            .registry
            .get_component_mut::<TransformComponent>(ctx.entity)
            .unwrap();

        let mut box_collider = ctx
            .registry
            .get_component_mut::<BoxColliderComponent>(ctx.entity)
            .unwrap();

        transform.position.set(&Vector2::new(
            self.resolution.width as f64 / 2.0,
            (self.resolution.height - 50) as f64,
        ));

        box_collider.size.as_mut().unwrap().x = 30.0;
    }

    fn on_update(&mut self, mut ctx: karakuri::components::Ctx) {
        let mut rigid_body = ctx
            .registry
            .get_component_mut::<RigidBodyComponent>(ctx.entity)
            .unwrap();
        let transform = ctx
            .registry
            .get_component::<TransformComponent>(ctx.entity)
            .unwrap();

        self.movement_handler(&ctx, &mut rigid_body);
        self.fire(
            &mut ctx,
            TransformComponent {
                position: transform.position.create_copy(),
                rotation: transform.rotation,
                scale: transform.scale.create_copy(),
            },
        );
    }

    fn on_collision(&mut self, other: &Entity, ctx: Ctx) {
        if let Some(other_tag) = ctx.registry.get_component::<TagComponent>(other) {
            if other_tag.value() == "enemy" {
                ctx.spawner.destroy_entity(ctx.entity.clone());
            }
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
