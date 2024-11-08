use kec::Registry;

use crate::{
    components::{CameraComponent, TransformComponent},
    errors::panic_queried,
};

#[derive(Debug, Default)]
pub struct CameraSystem {}

impl CameraSystem {
    pub fn update(&self, registry: &mut Registry) {
        let operators = registry.query().with_component::<CameraSystem>().build();

        if let Some(operator) = operators.first() {
            let camera = registry
                .get_component::<CameraComponent>(operator)
                .unwrap_or_else(|| panic_queried::<CameraComponent>(operator));

            match &camera.target {
                Some(target) => {
                    let mut transform = registry
                        .get_component_mut::<TransformComponent>(operator)
                        .unwrap_or_else(|| panic_queried::<TransformComponent>(operator));

                    let target_transform = registry
                        .get_component::<TransformComponent>(target)
                        .unwrap_or_else(|| panic_queried::<TransformComponent>(target));

                    transform.position.set(&target_transform.position);
                }
                None => {}
            }
        }
    }
}
