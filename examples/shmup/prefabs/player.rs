use std::cell::RefMut;

use karakuri::components::{
    Animation, AnimationControllerComponent, AnimationParams, BehaviorComponent,
    BoxColliderComponent, ComponentPayload, Ctx, RigidBodyComponent, SpriteComponent, TagComponent,
    TransformComponent,
};
use karakuri::ec::Entity;
use karakuri::math::Vector2;
use karakuri::utils::Size;
use karakuri::window::KeyboardKey;

use super::player_laser::player_laser_prefab;

pub fn player_prefab(resolution: &Size) -> ComponentPayload {
    ComponentPayload {
        transform: Some(TransformComponent {
            scale: Vector2::new(2.0, 2.0),
            ..Default::default()
        }),
        box_collider: Some(BoxColliderComponent::default()),
        rigid_body: Some(RigidBodyComponent::default()),
        tag: Some(TagComponent::new(String::from("player"))),
        sprite: Some(SpriteComponent {
            texture_name: "player-straight",
            clip_size: Some(Vector2::new(48.0, 58.0)),
            layer: 99,
            ..Default::default()
        }),
        behavior: Some(Box::new(Player::new(100.0, resolution.clone()))),
        animation_controller: Some(AnimationControllerComponent::new(vec![
            Animation::new(AnimationParams {
                name: "player-straight",
                texture_name: "player-straight",
                frame_count: 3,
                frame_rate: 6,
                looping: true,
            }),
            Animation::new(AnimationParams {
                name: "player-left",
                texture_name: "player-left",
                frame_count: 3,
                frame_rate: 6,
                looping: true,
            }),
            Animation::new(AnimationParams {
                name: "player-right",
                texture_name: "player-right",
                frame_count: 3,
                frame_rate: 6,
                looping: true,
            }),
        ])),
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

    fn movement_handler(
        &mut self,
        ctx: &Ctx,
        rigid_body: &mut RefMut<RigidBodyComponent>,
        animation_controller: &mut RefMut<AnimationControllerComponent>,
    ) {
        let speed = if ctx.input_processor.is_down(KeyboardKey::KEY_LEFT_SHIFT) {
            self.speed * 2.0
        } else {
            self.speed
        };

        if ctx.input_processor.is_down(KeyboardKey::KEY_W) {
            animation_controller.set_animation("player-straight");
            rigid_body.velocity.y = -speed * ctx.delta_time;
        }
        if ctx.input_processor.is_down(KeyboardKey::KEY_A) {
            animation_controller.set_animation("player-left");
            rigid_body.velocity.x = -speed * ctx.delta_time;
        }
        if ctx.input_processor.is_down(KeyboardKey::KEY_S) {
            animation_controller.set_animation("player-straight");
            rigid_body.velocity.y = speed * ctx.delta_time;
        }
        if ctx.input_processor.is_down(KeyboardKey::KEY_D) {
            animation_controller.set_animation("player-right");
            rigid_body.velocity.x = speed * ctx.delta_time;
        }
    }

    fn fire(&self, ctx: &mut Ctx, position: Vector2) {
        if ctx.input_processor.is_pressed(KeyboardKey::KEY_SPACE) {
            ctx.spawner.add_entity(player_laser_prefab(position));
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

        box_collider.size.as_mut().unwrap().x = 20.0;
        box_collider.size.as_mut().unwrap().y = 30.0;
        box_collider.position_offset.y = 5.0;
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
        let mut animation_controller = ctx
            .registry
            .get_component_mut::<AnimationControllerComponent>(ctx.entity)
            .unwrap();

        self.movement_handler(&ctx, &mut rigid_body, &mut animation_controller);
        self.fire(&mut ctx, transform.position.create_copy());
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
