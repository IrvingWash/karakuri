use kec::Registry;
use kmath::Vector2;

use crate::{
    components::{CameraComponent, TransformComponent},
    errors::panic_queried,
};

#[derive(Debug, Default)]
pub struct CameraSystem {}

impl CameraSystem {
    pub fn update(&self, registry: &mut Registry, resolution: &Vector2) {
        let operators = registry.query().with_component::<CameraComponent>().build();

        if let Some(operator) = operators.first() {
            let camera = registry
                .get_component::<CameraComponent>(operator)
                .unwrap_or_else(|| panic_queried::<CameraComponent>(operator));

            let mut transform = registry
                .get_component_mut::<TransformComponent>(operator)
                .unwrap_or_else(|| panic_queried::<TransformComponent>(operator));

            match &camera.target {
                Some(target) => {
                    let target_transform = registry
                        .get_component::<TransformComponent>(target)
                        .unwrap_or_else(|| panic_queried::<TransformComponent>(target));

                    transform.position = target_transform.position.clone();
                }
                None => {
                    transform.position = resolution.to_divided(2.0);
                }
            }
        }
    }
}
