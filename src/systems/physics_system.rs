use std::cell::Ref;

use kec::{Entity, Registry};
use kutils::collision::aabb_centered;

use crate::{
    adapters::InputProcessorAdapter,
    components::{
        BehaviorComponent, BoxColliderComponent, Ctx, FigureComponent, RigidBodyComponent,
        TransformComponent,
    },
    errors::panic_queried,
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
            .with_component::<FigureComponent>()
            .build();

        for i in 0..collidable_entities.len() {
            let entity = collidable_entities[i];

            for j in i + 1..collidable_entities.len() {
                let other = collidable_entities[j];

                let (transform, box_collider, figure) =
                    self.components_for_collision(&entity, registry);
                let (other_transform, other_box_collider, other_figure) =
                    self.components_for_collision(&other, registry);

                if aabb_centered(
                    &transform.position.to_added(&box_collider.position_offset),
                    &figure.size.to_scaled(&box_collider.size_scale),
                    &other_transform
                        .position
                        .to_added(&other_box_collider.position_offset),
                    &other_figure.size.to_scaled(&other_box_collider.size_scale),
                ) {
                    self.notify_collided_entity(
                        &entity,
                        &other,
                        registry,
                        delta_time,
                        input_processor,
                    );

                    self.notify_collided_entity(
                        &other,
                        &entity,
                        registry,
                        delta_time,
                        input_processor,
                    );
                }
            }
        }
    }

    fn notify_collided_entity<'a>(
        &self,
        entity: &Entity,
        other: &Entity,
        registry: &'a Registry,
        delta_time: f64,
        input_processor: &InputProcessorAdapter,
    ) {
        registry
            .get_component_mut::<Box<dyn BehaviorComponent>>(other)
            .unwrap_or_else(|| panic_queried::<dyn BehaviorComponent>(*other))
            .collide(
                &entity,
                Ctx {
                    delta_time,
                    entity: &other,
                    input_processor,
                    registry,
                },
            );
    }

    fn components_for_collision<'a>(
        &self,
        entity: &Entity,
        registry: &'a Registry,
    ) -> (
        Ref<'a, TransformComponent>,
        Ref<'a, BoxColliderComponent>,
        Ref<'a, FigureComponent>,
    ) {
        let transform = registry
            .get_component::<TransformComponent>(entity)
            .unwrap_or_else(|| panic_queried::<TransformComponent>(*entity));
        let box_collider = registry
            .get_component::<BoxColliderComponent>(entity)
            .unwrap_or_else(|| panic_queried::<BoxColliderComponent>(*entity));
        // TODO: This currently works only with figures, not with sprites
        // TODO: We should be able to collide even with invisible entities.
        // I think we must populate box_collider from sprite/figure as we populate sprite from texture in Scene
        let figure = registry
            .get_component::<FigureComponent>(entity)
            .unwrap_or_else(|| panic_queried::<FigureComponent>(*entity));

        (transform, box_collider, figure)
    }
}
