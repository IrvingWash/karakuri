use karakuri::components::{
    BehaviorComponent, BoxColliderComponent, ComponentPayload, TagComponent, TransformComponent,
};
use karakuri::ec::Entity;
use karakuri::math::Vector2;

pub fn laser_destroyer_prefab() -> ComponentPayload {
    ComponentPayload {
        transform: Some(TransformComponent {
            position: Vector2::new(400.0, 10.0),
            ..Default::default()
        }),
        box_collider: Some(BoxColliderComponent {
            size: Some(Vector2::new(800.0, 10.0)),
            ..Default::default()
        }),
        tag: Some(TagComponent::new(String::from("laser_destroyer"))),
        behavior: Some(Box::new(LaserDestroyer {})),
        ..Default::default()
    }
}

#[derive(Debug)]
struct LaserDestroyer {}

impl BehaviorComponent for LaserDestroyer {
    fn on_collision(&mut self, other: &Entity, ctx: karakuri::components::Ctx) {
        if let Some(other_tag) = ctx.registry.get_component::<TagComponent>(other) {
            if other_tag.value() == "player_laser" {
                ctx.spawner.destroy_entity(other.clone());
            }
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
