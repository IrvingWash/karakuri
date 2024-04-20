use crate::{
    components::{NameComponent, TransformComponent},
    core::{FpsController, Renderer},
    Entity,
};

const MAX_ENTITIES: usize = 5000;

pub struct ComponentsPayload {
    name_component: NameComponent,
    transform_component: Option<TransformComponent>,
}

struct EntityToAdd {
    id: usize,
    components: ComponentsPayload,
}

pub struct Scene {
    next_id: usize,
    free_ids: Vec<usize>,

    entities: [Option<Entity>; MAX_ENTITIES],
    name_components: [Option<NameComponent>; MAX_ENTITIES],
    transform_components: [Option<TransformComponent>; MAX_ENTITIES],

    entities_to_add: Vec<EntityToAdd>,
    entities_to_remove: Vec<Entity>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            next_id: 0,
            free_ids: Vec::new(),

            entities: [(); MAX_ENTITIES].map(|_| None),
            name_components: [(); MAX_ENTITIES].map(|_| None),
            transform_components: [(); MAX_ENTITIES].map(|_| None),

            entities_to_add: Vec::new(),
            entities_to_remove: Vec::new(),
        }
    }

    pub fn play(&mut self, fps_controller: &mut FpsController, renderer: &mut Renderer) {
        loop {
            let _delta_time = fps_controller.cap_framerate();

            self.sync_remove();
            self.sync_add();

            renderer.start_frame();
            renderer.finish_frame();
        }
    }

    pub fn add_entity(&mut self, components: ComponentsPayload) {
        let id = match self.free_ids.pop() {
            Some(id) => id,
            None => {
                self.next_id += 1;

                self.next_id - 1
            }
        };

        if id > MAX_ENTITIES {
            panic!("Too many entities");
        }

        self.entities_to_add.push(EntityToAdd { id, components });
    }

    fn sync_add(&mut self) {
        for entity_to_add in self.entities_to_add.drain(..) {
            let id = entity_to_add.id;

            self.entities[id] = Some(Entity::new(id));
            self.name_components[id] = Some(entity_to_add.components.name_component);
            self.transform_components[id] = entity_to_add.components.transform_component;
        }
    }

    fn sync_remove(&mut self) {
        for entity_to_remove in self.entities_to_remove.drain(..) {
            let mut has_removed = false;

            // Remove from entities to add
            self.entities_to_add.retain(|entity_to_add| {
                let result = entity_to_add.id != entity_to_remove.id();

                if result {
                    has_removed = true;
                }

                return result;
            });

            if has_removed {
                self.free_ids.push(entity_to_remove.id());

                continue;
            }

            // Remove from added entities
            let id = self.entities.iter().position(|entity| match entity {
                None => false,
                Some(entity) => entity.id() == entity_to_remove.id(),
            });

            match id {
                None => (),
                Some(id) => {
                    self.entities[id] = None;

                    self.free_ids.push(entity_to_remove.id());
                }
            }
        }
    }
}
