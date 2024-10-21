use kec::{EntityId, World};

use crate::components::ComponentPayload;

pub struct Scene {
    world: World,
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

    pub(crate) fn sync(&mut self) {
        for entity_to_add in self.entities_to_add.drain(..) {
            let id = self.world.create_entity();

            if let Some(tag) = entity_to_add.tag {
                self.world.add_component(id, tag);
            }

            if let Some(transform) = entity_to_add.transform {
                self.world.add_component(id, transform);
            }
        }

        for entity_to_remove in self.entities_to_remove.drain(..) {
            self.world.remove_entity(entity_to_remove);
        }
    }
}
