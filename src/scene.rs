use kec::{EntityId, World};

use crate::components::{Behavior, ComponentPayload};

pub struct Scene {
    pub(crate) world: World,
    entities_to_add: Vec<ComponentPayload>,
    entities_to_remove: Vec<EntityId>,
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}

impl Scene {
    pub fn new() -> Self {
        Self {
            world: World::new(),
            entities_to_add: Vec::new(),
            entities_to_remove: Vec::new(),
        }
    }

    pub fn create_entity(&mut self, components: ComponentPayload) {
        self.entities_to_add.push(components);
    }

    pub fn remove_entity(&mut self, id: EntityId) {
        self.entities_to_remove.push(id);
    }

    #[allow(dead_code)]
    fn sync(&mut self) {
        // Remove
        for entity_to_remove in self.entities_to_remove.drain(..) {
            if let Some(mut behavior) = self
                .world
                .get_component_mut::<Box<dyn Behavior>>(entity_to_remove)
            {
                behavior.destroy();
            }

            self.world.remove_entity(entity_to_remove);
        }

        // Add
        let mut entities_to_start: Vec<EntityId> = Vec::new();

        for entity_to_add in self.entities_to_add.drain(..) {
            let id = self.world.create_entity();

            if let Some(tag) = entity_to_add.tag {
                self.world.add_component(id, tag);
            }

            if let Some(transform) = entity_to_add.transform {
                self.world.add_component(id, transform);
            }

            if let Some(behavior) = entity_to_add.behavior {
                entities_to_start.push(id);

                self.world.add_component(id, behavior);
            }

            if let Some(sprite) = entity_to_add.sprite {
                self.world.add_component(id, sprite);
            }
        }

        for entity_to_start in entities_to_start {
            self.world
                .get_component_mut::<Box<dyn Behavior>>(entity_to_start)
                .unwrap()
                .start();
        }
    }
}
