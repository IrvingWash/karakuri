use crate::{
    components::{BehaviorComponent, NameComponent, ShapeComponent, TransformComponent},
    core::{FpsController, InputController, Renderer},
    Entity,
};

const MAX_ENTITIES: usize = 5000;

pub struct ComponentsPayload {
    pub name_component: NameComponent,
    pub transform_component: Option<TransformComponent>,
    pub shape_component: Option<ShapeComponent>,
    pub behavior_component: Option<Box<dyn BehaviorComponent>>,
}

pub struct Spawner {
    components_payloads: Vec<ComponentsPayload>,
    entities_to_remove: Vec<Entity>,
}

impl Spawner {
    fn new() -> Spawner {
        Spawner {
            components_payloads: Vec::new(),
            entities_to_remove: Vec::new(),
        }
    }

    pub fn add_entity(&mut self, components: ComponentsPayload) {
        self.components_payloads.push(components);
    }

    pub fn remove_entity(&mut self, entity: Entity) {
        self.entities_to_remove.push(entity);
    }
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
    shape_components: [Option<ShapeComponent>; MAX_ENTITIES],
    behavior_components: [Option<Box<dyn BehaviorComponent>>; MAX_ENTITIES],

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
            shape_components: [(); MAX_ENTITIES].map(|_| None),
            behavior_components: [(); MAX_ENTITIES].map(|_| None),

            entities_to_add: Vec::new(),
            entities_to_remove: Vec::new(),
        }
    }

    pub fn play(
        &mut self,
        fps_controller: &mut FpsController,
        renderer: &mut Renderer,
        input_controller: &mut InputController,
    ) {
        loop {
            // Prepare
            let delta_time = fps_controller.cap_framerate();

            input_controller.process();
            let input_result = input_controller.result();
            if input_result.should_quit {
                break;
            }

            self.sync_remove();
            self.sync_add();

            // Update
            let mut spawner = Spawner::new();

            for behavior in &mut self.behavior_components {
                match behavior {
                    None => continue,
                    Some(behavior) => behavior.update(
                        input_result,
                        delta_time,
                        &mut spawner,
                        &self.name_components,
                        &mut self.transform_components,
                    ),
                }
            }

            for components_payload in spawner.components_payloads.drain(..) {
                self.add_entity(components_payload);
            }
            for entity_to_remove in spawner.entities_to_remove.drain(..) {
                self.entities_to_remove.push(entity_to_remove);
            }

            // Render
            renderer.start_frame();
            for entity in &self.entities {
                match entity {
                    None => continue,
                    Some(entity) => {
                        let transform = self.transform_components[entity.id()].as_ref().unwrap();
                        let shape = self.shape_components[entity.id()].as_ref().unwrap();

                        renderer.filled_rectangle(&transform.position, &shape.size, &shape.color);
                    }
                }
            }
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
        let mut ids_to_start: Vec<usize> = Vec::new();

        for entity_to_add in self.entities_to_add.drain(..) {
            let id = entity_to_add.id;

            self.entities[id] = Some(Entity::new(id));
            self.name_components[id] = Some(entity_to_add.components.name_component);
            self.transform_components[id] = entity_to_add.components.transform_component;
            self.shape_components[id] = entity_to_add.components.shape_component;
            self.behavior_components[id] = entity_to_add.components.behavior_component;

            ids_to_start.push(id);
        }

        for id in ids_to_start {
            let behavior = &mut self.behavior_components[id];

            match behavior {
                None => continue,
                Some(behavior) => behavior.start(&self.name_components),
            }
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

                result
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
                    self.name_components[id] = None;
                    self.transform_components[id] = None;
                    self.shape_components[id] = None;
                    self.behavior_components[id] = None;

                    self.free_ids.push(entity_to_remove.id());
                }
            }
        }
    }
}
