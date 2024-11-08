use karakuri::components::{
    Animation, AnimationControllerComponent, AnimationParams, BehaviorComponent,
    BoxColliderComponent, ComponentPayload, RigidBodyComponent, SpriteComponent, TagComponent,
    TransformComponent,
};
use karakuri::ec::Entity;
use karakuri::math::Vector2;
use karakuri::{EventBundle, UpdateContext};

use super::enemy_laser_prefab;

pub fn enemy_prefab(position: Vector2) -> ComponentPayload {
    ComponentPayload {
        transform: Some(TransformComponent {
            position,
            scale: Vector2::new(2.0, 2.0),
            ..Default::default()
        }),
        box_collider: Some(BoxColliderComponent::default()),
        rigid_body: Some(RigidBodyComponent {
            velocity: Vector2::new(if rand::random() { 1.0 } else { -1.0 }, 1.0),
        }),
        tag: Some(TagComponent::new(String::from("enemy"))),
        sprite: Some(SpriteComponent {
            texture_name: "enemy-straight",
            clip_size: Some(Vector2::new(48.0, 48.0)),
            layer: 3,
            ..Default::default()
        }),
        behavior: Some(Box::new(Enemy::new())),
        animation_controller: Some(AnimationControllerComponent::new(vec![
            Animation::new(AnimationParams {
                frame_count: 3,
                frame_rate: 6,
                looping: true,
                name: "enemy-straight",
                texture_name: "enemy-straight",
            }),
            Animation::new(AnimationParams {
                frame_count: 8,
                frame_rate: 8,
                name: "explosion",
                texture_name: "explosion",
                looping: true,
            }),
        ])),
        ..Default::default()
    }
}

#[derive(Debug)]
struct Enemy {
    explosion_timer: i64,
    shooting_timer: i64,
    is_destroying: bool,
    player: Option<Entity>,
}

impl Enemy {
    fn new() -> Self {
        Self {
            explosion_timer: -1,
            shooting_timer: -1,
            is_destroying: false,
            player: None,
        }
    }
}

impl BehaviorComponent for Enemy {
    fn on_start(&mut self, ctx: UpdateContext) {
        let mut box_collider = ctx
            .registry
            .get_component_mut::<BoxColliderComponent>(ctx.entity)
            .unwrap();

        box_collider.size.as_mut().unwrap().x = 30.0;
        box_collider.size.as_mut().unwrap().y = 25.0;

        self.shooting_timer = ctx.timer.set_interval(1000.0) as i64;

        self.player = ctx
            .registry
            .find_entity(&TagComponent::new(String::from("player")));
    }

    fn on_update(&mut self, ctx: UpdateContext) {
        if let Some(player) = &self.player {
            if ctx.registry.is_alive(player) {
                self.player.as_ref().unwrap();
            }
        }
    }

    fn on_collision(&mut self, other: &Entity, ctx: UpdateContext) {
        if let Some(other_tag) = ctx.registry.get_component::<TagComponent>(other) {
            if other_tag.value() == "player_laser" && !self.is_destroying {
                let mut animation_controller = ctx
                    .registry
                    .get_component_mut::<AnimationControllerComponent>(ctx.entity)
                    .unwrap();

                animation_controller.set_animation("explosion");

                self.explosion_timer = ctx.timer.set_timeout(1000.0) as i64;

                ctx.timer.clear_interval(self.shooting_timer as usize);

                self.is_destroying = true;
            }
        }
    }

    fn on_events(&mut self, events: &EventBundle, ctx: UpdateContext) {
        if events
            .finished_timers
            .contains(&(self.shooting_timer as usize))
        {
            let transform = ctx
                .registry
                .get_component::<TransformComponent>(ctx.entity)
                .unwrap();

            ctx.spawner
                .add_entity(enemy_laser_prefab(transform.position.clone()));
        }

        if events
            .finished_timers
            .contains(&(self.explosion_timer as usize))
        {
            ctx.spawner.destroy_entity(ctx.entity.clone());
        }

        if events.custom_events.contains("player_died") {
            ctx.timer.clear_interval(self.shooting_timer as usize);
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
