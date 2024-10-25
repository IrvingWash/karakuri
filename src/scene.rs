use kec::{Entity, Registry};

use crate::components::{BehaviorComponent, ComponentPayload};

#[derive(Debug, Default)]
pub struct Scene {
    entities_to_add: Vec<ComponentPayload>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            entities_to_add: Vec::new(),
        }
    }

    pub fn create_initial_entities(
        &mut self,
        registry: &mut Registry,
        entities: Vec<ComponentPayload>,
    ) {
        for entity in entities {
            self.create_entity(entity);
        }

        self.sync(registry);
    }

    fn create_entity(&mut self, component_payload: ComponentPayload) {
        self.entities_to_add.push(component_payload);
    }

    fn sync(&mut self, registry: &mut Registry) {
        let mut entities_to_start: Vec<Entity> = Vec::new();

        for bundle in self.entities_to_add.drain(..) {
            let entity = registry.create_entity();

            if let Some(transform) = bundle.transform {
                registry.add_component(&entity, transform);
            }

            if let Some(behavior) = bundle.behavior {
                registry.add_component(&entity, behavior);
                entities_to_start.push(entity);
            }

            if let Some(tab) = bundle.tag {
                registry.add_component(&entity, tab);
            }

            if let Some(sprite) = bundle.sprite {
                registry.add_component(&entity, sprite);
            }
        }

        for entity in entities_to_start {
            registry
                .get_component_mut::<Box<dyn BehaviorComponent>>(&entity)
                .unwrap()
                .start();
        }
    }
}
