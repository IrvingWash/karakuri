use std::cell::Ref;

use kec::{Entity, Registry};
use kmath::Vector2;
use kutils::collision::aabb_centered;

use crate::{
    adapters::{InputProcessorAdapter, TimerAdapter},
    components::{
        BehaviorComponent, BoxColliderComponent, Ctx, RigidBodyComponent, TransformComponent,
    },
    errors::{panic_queried, panic_uninitialized_collider},
    spawner::Spawner,
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
        spawner: &mut Spawner,
        timer: &mut TimerAdapter,
    ) {
        self.move_entities(registry, delta_time);
        self.collide(registry, delta_time, input_processor, spawner, timer);
    }

    fn move_entities(&self, registry: &mut Registry, delta_time: f64) {
        let affected_entities = registry
            .query()
            .with_component::<TransformComponent>()
            .with_component::<RigidBodyComponent>()
            .build();

        for entity in &affected_entities {
            let mut transform = registry
                .get_component_mut::<TransformComponent>(entity)
                .unwrap_or_else(|| panic_queried::<TransformComponent>(entity));
            let rigid_body = registry
                .get_component::<RigidBodyComponent>(entity)
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
        spawner: &mut Spawner,
        timer: &mut TimerAdapter,
    ) {
        let collidable_entities = registry
            .query()
            .with_component::<TransformComponent>()
            .with_component::<BoxColliderComponent>()
            .build();

        // Looks like this is O(n)
        for i in 0..collidable_entities.len() {
            let entity = &collidable_entities[i];

            for other in collidable_entities.iter().skip(i + 1) {
                let (transform, box_collider) = self.components_for_collision(entity, registry);
                let (other_transform, other_box_collider) =
                    self.components_for_collision(other, registry);

                if aabb_centered(
                    &self.create_position_for_collision(&transform, &box_collider),
                    &box_collider
                        .size
                        .as_ref()
                        .unwrap_or_else(|| panic_uninitialized_collider("size"))
                        .to_scaled_by_other(&transform.scale),
                    &self.create_position_for_collision(&other_transform, &other_box_collider),
                    &other_box_collider
                        .size
                        .as_ref()
                        .unwrap_or_else(|| panic_uninitialized_collider("size"))
                        .to_scaled_by_other(&other_transform.scale),
                ) {
                    self.notify_collided_entity(
                        entity,
                        other,
                        registry,
                        delta_time,
                        input_processor,
                        spawner,
                        timer,
                    );

                    self.notify_collided_entity(
                        other,
                        entity,
                        registry,
                        delta_time,
                        input_processor,
                        spawner,
                        timer,
                    );
                }
            }
        }
    }

    fn create_position_for_collision(
        &self,
        transform: &Ref<TransformComponent>,
        box_collider: &Ref<BoxColliderComponent>,
    ) -> Vector2 {
        let mut temp_box_collider_size = box_collider.size.as_ref().unwrap().create_copy();
        temp_box_collider_size.scale_by_other(&transform.scale);
        temp_box_collider_size.divide(2.0);

        let mut temp_position = transform.position.create_copy();
        temp_position.add(&box_collider.position_offset);
        temp_position.subtract(&temp_box_collider_size);

        temp_position
    }

    #[allow(clippy::too_many_arguments)]
    fn notify_collided_entity(
        &self,
        entity: &Entity,
        other: &Entity,
        registry: &Registry,
        delta_time: f64,
        input_processor: &InputProcessorAdapter,
        spawner: &mut Spawner,
        timer: &mut TimerAdapter,
    ) {
        if let Some(mut behavior) = registry.get_dyn_component_mut::<dyn BehaviorComponent>(other) {
            behavior.collide(
                entity,
                Ctx {
                    delta_time,
                    entity: other,
                    input_processor,
                    registry,
                    spawner,
                    timer,
                },
            );
        }
    }

    fn components_for_collision<'a>(
        &self,
        entity: &Entity,
        registry: &'a Registry,
    ) -> (Ref<'a, TransformComponent>, Ref<'a, BoxColliderComponent>) {
        let transform = registry
            .get_component::<TransformComponent>(entity)
            .unwrap_or_else(|| panic_queried::<TransformComponent>(entity));
        let box_collider = registry
            .get_component::<BoxColliderComponent>(entity)
            .unwrap_or_else(|| panic_queried::<BoxColliderComponent>(entity));

        (transform, box_collider)
    }
}
