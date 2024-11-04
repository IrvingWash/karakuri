use karakuri::components::{
    BehaviorComponent, BoxColliderComponent, ComponentPayload, TagComponent, TransformComponent,
};
use karakuri::ec::Entity;
use karakuri::math::Vector2;

#[derive(Debug, PartialEq)]
pub enum LaserDestroyerPosition {
    Top,
    Bottom,
}

pub fn laser_destroyer_prefab(position: LaserDestroyerPosition) -> ComponentPayload {
    ComponentPayload {
        transform: Some(TransformComponent {
            position: Vector2::new(
                400.0,
                if position == LaserDestroyerPosition::Top {
                    10.0
                } else {
                    200.0
                },
            ),
            ..Default::default()
        }),
        box_collider: Some(BoxColliderComponent {
            size: Some(Vector2::new(800.0, 10.0)),
            ..Default::default()
        }),
        tag: Some(TagComponent::new(String::from("laser_destroyer"))),
        behavior: Some(Box::new(LaserDestroyer { position })),
        ..Default::default()
    }
}

#[derive(Debug)]
struct LaserDestroyer {
    position: LaserDestroyerPosition,
}

impl BehaviorComponent for LaserDestroyer {
    fn on_collision(&mut self, other: &Entity, ctx: karakuri::components::Ctx) {
        if let Some(other_tag) = ctx.registry.get_component::<TagComponent>(other) {
            if other_tag.value()
                == if self.position == LaserDestroyerPosition::Top {
                    "player_laser"
                } else {
                    "enemy_laser"
                }
            {
                ctx.spawner.destroy_entity(other.clone());
            }
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
