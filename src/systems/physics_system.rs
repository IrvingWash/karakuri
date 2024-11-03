use std::cell::Ref;

use kec::{Entity, Registry};
use kmath::Vector2;
use kutils::collision::aabb_centered;

use crate::{
    adapters::InputProcessorAdapter,
    components::{
        BehaviorComponent, BoxColliderComponent, Ctx, RigidBodyComponent, TransformComponent,
    },
    errors::{panic_queried, panic_uninitialized_collider},
};

pub struct PhysicsSystem {}

impl PhysicsSystem {
    pub fn new() -> Self {
        Self {}
    }

    pub fn affect(
        &self,
        registry: &mut Registry,
        delta_time: f64,
        input_processor: &InputProcessorAdapter,
    ) {
        self.move_entities(registry, delta_time);
        self.collide(registry, delta_time, input_processor);
    }

    fn move_entities(&self, registry: &mut Registry, delta_time: f64) {
        let affected_entities = registry
            .query()
            .with_component::<TransformComponent>()
            .with_component::<RigidBodyComponent>()
            .build();

        for entity in affected_entities {
            let mut transform = registry
                .get_component_mut::<TransformComponent>(&entity)
                .unwrap_or_else(|| panic_queried::<TransformComponent>(entity));
            let rigid_body = registry
                .get_component::<RigidBodyComponent>(&entity)
                .unwrap_or_else(|| panic_queried::<RigidBodyComponent>(entity));

            transform
                .position
                .add(&rigid_body.velocity.to_scaled(delta_time));
        }
    }

    fn collide(
        &self,
        registry: &mut Registry,
        delta_time: f64,
        input_processor: &InputProcessorAdapter,
    ) {
        let collidable_entities = registry
            .query()
            .with_component::<TransformComponent>()
            .with_component::<BoxColliderComponent>()
            .build();

        // Looks like this is O(n)
        for i in 0..collidable_entities.len() {
            let entity = collidable_entities[i];

            for other in collidable_entities.iter().skip(i + 1) {
                let (transform, box_collider) = self.components_for_collision(&entity, registry);
                let (other_transform, other_box_collider) =
                    self.components_for_collision(other, registry);

                let position = transform.position.to_added(&box_collider.position_offset);
                let other_position = other_transform
                    .position
                    .to_added(&other_box_collider.position_offset);

                if aabb_centered(
                    &Vector2::new(
                        &position.x
                            - box_collider.size.as_ref().unwrap().x * transform.scale.x / 2.0,
                        &position.y
                            - box_collider.size.as_ref().unwrap().y * transform.scale.y / 2.0,
                    ),
                    &box_collider
                        .size
                        .as_ref()
                        .unwrap_or_else(|| panic_uninitialized_collider("size"))
                        .to_scaled_by_other(&transform.scale),
                    &Vector2::new(
                        &other_position.x
                            - other_box_collider.size.as_ref().unwrap().x * other_transform.scale.x
                                / 2.0,
                        &other_position.y
                            - other_box_collider.size.as_ref().unwrap().y * other_transform.scale.y
                                / 2.0,
                    ),
                    &other_box_collider
                        .size
                        .as_ref()
                        .unwrap_or_else(|| panic_uninitialized_collider("size"))
                        .to_scaled_by_other(&other_transform.scale),
                ) {
                    self.notify_collided_entity(
                        &entity,
                        other,
                        registry,
                        delta_time,
                        input_processor,
                    );

                    self.notify_collided_entity(
                        other,
                        &entity,
                        registry,
                        delta_time,
                        input_processor,
                    );
                }
            }
        }
    }

    fn notify_collided_entity(
        &self,
        entity: &Entity,
        other: &Entity,
        registry: &Registry,
        delta_time: f64,
        input_processor: &InputProcessorAdapter,
    ) {
        registry
            .get_component_mut::<Box<dyn BehaviorComponent>>(other)
            .unwrap_or_else(|| panic_queried::<dyn BehaviorComponent>(*other))
            .collide(
                entity,
                Ctx {
                    delta_time,
                    entity: other,
                    input_processor,
                    registry,
                },
            );
    }

    fn components_for_collision<'a>(
        &self,
        entity: &Entity,
        registry: &'a Registry,
    ) -> (Ref<'a, TransformComponent>, Ref<'a, BoxColliderComponent>) {
        let transform = registry
            .get_component::<TransformComponent>(entity)
            .unwrap_or_else(|| panic_queried::<TransformComponent>(*entity));
        let box_collider = registry
            .get_component::<BoxColliderComponent>(entity)
            .unwrap_or_else(|| panic_queried::<BoxColliderComponent>(*entity));

        (transform, box_collider)
    }
}
