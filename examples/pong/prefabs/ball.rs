use std::any::Any;

use karakuri::components::{
    BehaviorComponent, BoxColliderComponent, ComponentPayload, RigidBodyComponent, SpriteComponent,
    TagComponent, TransformComponent,
};
use karakuri::ec::Entity;
use karakuri::math::Vector2;
use karakuri::UpdateContext;
use kwindow::KeyboardKey;

use super::paddle::PaddleSide;

pub fn ball_prefab(resolution: &Vector2) -> ComponentPayload {
    ComponentPayload {
        sprite: Some(SpriteComponent {
            texture_name: "square",
            layer: 0,
            ..Default::default()
        }),
        tag: Some(TagComponent::new(String::from("ball"))),
        transform: Some(TransformComponent {
            position: Vector2::new(resolution.x / 2.0, resolution.y / 2.0),
            scale: Vector2::new(0.7, 0.7),
            ..Default::default()
        }),
        behavior: Some(Box::new(Ball {
            speed: 40.0,
            resolution: resolution.clone(),
        })),
        rigid_body: Some(RigidBodyComponent::default()),
        box_collider: Some(BoxColliderComponent::default()),
        ..Default::default()
    }
}

#[derive(Default, Debug)]
struct Ball {
    speed: f64,
    resolution: Vector2,
}

impl BehaviorComponent for Ball {
    fn on_update(&mut self, ctx: UpdateContext) {
        if ctx.input_processor.is_pressed(KeyboardKey::KEY_SPACE) {
            let mut rigid_body = ctx
                .registry
                .get_component_mut::<RigidBodyComponent>(ctx.entity)
                .unwrap();

            let x_modifier = if rand::random() { -1.0 } else { 1.0 };
            let y_modifier = if rand::random() { -1.0 } else { 1.0 };

            rigid_body.velocity.set(&Vector2::new(
                self.speed * x_modifier,
                self.speed * y_modifier,
            ));
        }

        let transform = ctx
            .registry
            .get_component::<TransformComponent>(&ctx.entity)
            .unwrap();
        let mut rigid_body = ctx
            .registry
            .get_component_mut::<RigidBodyComponent>(ctx.entity)
            .unwrap();

        // Collide manually with the top and bottom of the screen
        if transform.position.y <= 0.0 || transform.position.y >= self.resolution.y {
            rigid_body.velocity.y *= -1.0;
        }

        // Destroy self if collided with right or left of the screen
        if transform.position.x <= 0.0 || transform.position.y >= self.resolution.x {
            ctx.spawner.destroy_entity(ctx.entity.clone());
        }
    }

    fn on_collision(&mut self, other: &Entity, ctx: UpdateContext) {
        if let Some(other_tag) = ctx.registry.get_component::<TagComponent>(other) {
            if *other_tag.value() == PaddleSide::Left.to_string()
                || *other_tag.value() == PaddleSide::Right.to_string()
            {
                let mut rigid_body = ctx
                    .registry
                    .get_component_mut::<RigidBodyComponent>(ctx.entity)
                    .unwrap();

                rigid_body.velocity.x *= -1.0;

                if rand::random() {
                    rigid_body.velocity.y *= -1.0;
                }
            }
        }
    }

    fn on_destroy(&mut self, _ctx: UpdateContext) {
        println!("I (ball) am sadly destroyed :(");
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
