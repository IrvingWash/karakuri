use karakuri::components::{
    BehaviorComponent, BoxColliderComponent, ComponentPayload, RigidBodyComponent, SpriteComponent,
    TagComponent, TransformComponent,
};
use karakuri::ec::Entity;
use karakuri::math::Vector2;

pub fn enemy_prefab(position: Vector2) -> ComponentPayload {
    ComponentPayload {
        transform: Some(TransformComponent::from_position(position)),
        box_collider: Some(BoxColliderComponent::default()),
        rigid_body: Some(RigidBodyComponent {
            velocity: Vector2::new(1.0, 1.0),
        }),
        tag: Some(TagComponent::new(String::from("enemy"))),
        sprite: Some(SpriteComponent::from_texture_name("enemy_red")),
        behavior: Some(Box::new(Enemy {})),
        ..Default::default()
    }
}

#[derive(Debug)]
struct Enemy {}

impl BehaviorComponent for Enemy {
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
