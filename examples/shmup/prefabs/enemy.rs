use karakuri::components::{
    Animation, AnimationControllerComponent, AnimationParams, BehaviorComponent,
    BoxColliderComponent, ComponentPayload, RigidBodyComponent, SpriteComponent, TagComponent,
    TransformComponent,
};
use karakuri::ec::Entity;
use karakuri::math::Vector2;

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
            layer: 99,
            ..Default::default()
        }),
        behavior: Some(Box::new(Enemy {})),
        animation_controller: Some(AnimationControllerComponent::new(vec![Animation::new(
            AnimationParams {
                frame_count: 3,
                frame_rate: 6,
                looping: true,
                name: "enemy-straight",
                texture_name: "enemy-straight",
            },
        )])),
    }
}

#[derive(Debug)]
struct Enemy {}

impl BehaviorComponent for Enemy {
    fn on_start(&mut self, ctx: karakuri::components::Ctx) {
        let mut box_collider = ctx
            .registry
            .get_component_mut::<BoxColliderComponent>(ctx.entity)
            .unwrap();

        box_collider.size.as_mut().unwrap().x = 30.0;
        box_collider.size.as_mut().unwrap().y = 25.0;
    }

    fn on_collision(&mut self, other: &Entity, ctx: karakuri::components::Ctx) {
        if let Some(other_tag) = ctx.registry.get_component::<TagComponent>(other) {
            if other_tag.value() == "player_laser" {
                ctx.spawner.destroy_entity(ctx.entity.clone());
            }
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
