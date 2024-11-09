use std::cell::Ref;

use kec::{Entity, Registry};
use kmath::Vector2;
use kutils::collision::aabb;
use kwindow::{InputProcessor, Timer, WindowCtx};

use crate::{
    adapters::{EventSenderAdapter, InputProcessorAdapter, RegistryAdapter, TimerAdapter},
    components::{BehaviorComponent, BoxColliderComponent, RigidBodyComponent, TransformComponent},
    errors::{panic_queried, panic_uninitialized_collider},
    event_buss::EventBuss,
    spawner::Spawner,
    update_context::UpdateContext,
};

#[derive(Debug, Default)]
pub struct PhysicsSystem {}

pub struct AffectParams<'a> {
    pub registry: &'a mut Registry,
    pub delta_time: f64,
    pub input_processor: &'a InputProcessor,
    pub spawner: &'a mut Spawner,
    pub timer: &'a mut Timer,
    pub ctx: &'a WindowCtx,
    pub event_buss: &'a mut EventBuss,
}

impl PhysicsSystem {
    #[allow(clippy::too_many_arguments)]
    #[inline]
    pub fn affect(&self, params: AffectParams) {
        self.move_entities(params.registry, params.delta_time);
        self.collide_entities(params);
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

    #[allow(clippy::too_many_arguments)]
    fn collide_entities(&self, mut params: AffectParams) {
        let collidable_entities = params
            .registry
            .query()
            .with_component::<TransformComponent>()
            .with_component::<BoxColliderComponent>()
            .build();

        // Looks like this is O(n)
        for i in 0..collidable_entities.len() {
            let entity = &collidable_entities[i];

            for other in collidable_entities.iter().skip(i + 1) {
                let (transform, box_collider) =
                    self.components_for_collision(entity, params.registry);
                let (other_transform, other_box_collider) =
                    self.components_for_collision(other, params.registry);

                if aabb(
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
                    drop(transform);
                    drop(box_collider);
                    drop(other_transform);
                    drop(other_box_collider);

                    self.notify_collided_entity(entity, other, &mut params);

                    self.notify_collided_entity(other, entity, &mut params);
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
    fn notify_collided_entity(&self, entity: &Entity, other: &Entity, params: &mut AffectParams) {
        if let Some(mut behavior) = params
            .registry
            .get_component_mut::<Box<dyn BehaviorComponent>>(other)
        {
            behavior.collide(
                entity,
                UpdateContext {
                    delta_time: params.delta_time,
                    entity: other,
                    registry: &RegistryAdapter::new(params.registry),
                    input_processor: &InputProcessorAdapter::new(
                        params.input_processor,
                        params.ctx,
                    ),
                    spawner: params.spawner,
                    timer: &mut TimerAdapter::new(params.timer),
                    event_sender: &mut EventSenderAdapter::new(params.event_buss),
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
