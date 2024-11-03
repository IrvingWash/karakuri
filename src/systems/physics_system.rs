use kec::Registry;
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

        for entity in &collidable_entities {
            for other in &collidable_entities {
                if entity == other {
                    continue;
                }

                let transform = registry
                    .get_component::<TransformComponent>(entity)
                    .unwrap_or_else(|| panic_queried::<TransformComponent>(*entity));
                let box_collider = registry
                    .get_component::<BoxColliderComponent>(entity)
                    .unwrap_or_else(|| panic_queried::<BoxColliderComponent>(*entity));
                // TODO: This currently works only with figures, not with sprites
                let figure = registry
                    .get_component::<FigureComponent>(entity)
                    .unwrap_or_else(|| panic_queried::<FigureComponent>(*entity));

                let other_transform = registry
                    .get_component::<TransformComponent>(other)
                    .unwrap_or_else(|| panic_queried::<TransformComponent>(*other));
                let other_box_collider = registry
                    .get_component::<BoxColliderComponent>(other)
                    .unwrap_or_else(|| panic_queried::<BoxColliderComponent>(*other));
                let other_figure = registry
                    .get_component::<FigureComponent>(other)
                    .unwrap_or_else(|| panic_queried::<FigureComponent>(*other));

                if aabb_centered(
                    &transform.position.to_added(&box_collider.position_offset),
                    &figure.size.to_scaled(&box_collider.size_scale),
                    &other_transform
                        .position
                        .to_added(&other_box_collider.position_offset),
                    &other_figure.size.to_scaled(&other_box_collider.size_scale),
                ) {
                    registry
                        .get_component_mut::<Box<dyn BehaviorComponent>>(entity)
                        .unwrap_or_else(|| panic_queried::<dyn BehaviorComponent>(*entity))
                        .collide(
                            other,
                            Ctx {
                                delta_time,
                                entity,
                                input_processor,
                                registry,
                            },
                        );
                }
            }
        }
    }
}
