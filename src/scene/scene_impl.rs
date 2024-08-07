use std::mem;

use crate::{
    components::{Behavior, ComponentsCtx, ComponentsPayload, Figure, Name, Transform},
    core::{FpsController, InputController, Renderer},
    Entity,
};

use super::Spawner;

// TODO: this should be configurable by user.
const MAX_ENTITIES: usize = 5000;

pub struct Scene {
    next_entity: Entity,
    free_entities: Vec<Entity>,

    entities: [Option<Entity>; MAX_ENTITIES],
    name_components: [Option<Name>; MAX_ENTITIES],
    transform_components: [Option<Transform>; MAX_ENTITIES],
    figure_components: [Option<Figure>; MAX_ENTITIES],
    behavior_components: [Option<Box<dyn Behavior>>; MAX_ENTITIES],

    spawner: Spawner,
}

impl Default for Scene {
    fn default() -> Scene {
        Scene::new()
    }
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            next_entity: 0,
            free_entities: Vec::new(),

            entities: [(); MAX_ENTITIES].map(|_| None),
            name_components: [(); MAX_ENTITIES].map(|_| None),
            transform_components: [(); MAX_ENTITIES].map(|_| None),
            figure_components: [(); MAX_ENTITIES].map(|_| None),
            behavior_components: [(); MAX_ENTITIES].map(|_| None),

            spawner: Spawner::new(),
        }
    }

    pub fn play(
        &mut self,
        fps_controller: &mut FpsController,
        input_controller: &mut InputController,
        renderer: &mut Renderer,
    ) {
        loop {
            // Prepare
            let delta_time = fps_controller.cap_framerate();

            input_controller.process();
            let input_result = input_controller.result();
            if input_result.should_quit {
                break;
            }

            // Update
            self.add_entities();
            self.remove_entities();

            for behavior in &mut self.behavior_components {
                match behavior {
                    None => continue,
                    Some(behavior) => behavior.update(
                        delta_time,
                        input_result,
                        &mut self.spawner,
                        ComponentsCtx {
                            names: &self.name_components,
                            transforms: &mut self.transform_components,
                            figures: &self.figure_components,
                        },
                    ),
                }
            }

            // Render
            renderer.start_frame();
            for entity in self.entities {
                match entity {
                    None => continue,
                    Some(entity) => {
                        let transform = self.transform_components[entity].as_ref().unwrap();
                        let figure = self.figure_components[entity].as_ref().unwrap(); // TODO: Can't unwrap!

                        renderer.filled_rectangle(&transform.position, &figure.size, &figure.color);
                    }
                }
            }
            renderer.finish_frame();
        }
    }

    pub fn add_initial_entities(&mut self, component_payloads: Vec<ComponentsPayload>) {
        let payloads_count = component_payloads.len();

        let mut entities_to_start = Vec::new();

        for (entity, payload) in component_payloads.into_iter().enumerate() {
            if payloads_count >= MAX_ENTITIES {
                panic!(
                    "{} entities were provided while only {} can exist",
                    payloads_count, MAX_ENTITIES
                );
            };

            self.populate_entity(entity, payload);
            entities_to_start.push(entity);
        }

        self.next_entity = payloads_count;

        self.start_entities(entities_to_start);
    }

    fn add_entities(&mut self) {
        let mut entities_to_start = Vec::new();

        for payload in mem::take(&mut self.spawner.entities_to_add) {
            let entity = self.free_entities.pop().unwrap_or_else(|| {
                self.next_entity += 1;

                self.next_entity - 1
            });

            if entity >= MAX_ENTITIES {
                panic!("Too many entities");
            }

            self.populate_entity(entity, payload);

            entities_to_start.push(entity);
        }

        self.start_entities(entities_to_start);
    }

    fn remove_entities(&mut self) {
        for entity_to_remove in mem::take(&mut self.spawner.entities_to_remove) {
            let entity = self.entities.iter().position(|entity| match entity {
                None => false,
                Some(entity) => *entity == entity_to_remove,
            });

            match entity {
                None => (),
                Some(entity) => self.depopulate_entity(entity),
            }
        }
    }

    fn start_entities(&mut self, entities_to_start: Vec<Entity>) {
        for entity in entities_to_start {
            let behavior = &mut self.behavior_components[entity];

            match behavior {
                None => continue,
                Some(behavior) => behavior.start(ComponentsCtx {
                    names: &self.name_components,
                    transforms: &mut self.transform_components,
                    figures: &self.figure_components,
                }),
            }
        }
    }

    fn populate_entity(&mut self, entity: Entity, payload: ComponentsPayload) {
        self.entities[entity] = Some(entity);
        self.name_components[entity] = Some(payload.name);
        self.transform_components[entity] = Some(payload.transform);
        self.behavior_components[entity] = payload.behavior;
        self.figure_components[entity] = payload.figure;
    }

    fn depopulate_entity(&mut self, entity: Entity) {
        self.entities[entity] = None;
        self.name_components[entity] = None;
        self.transform_components[entity] = None;
        self.behavior_components[entity] = None;
        self.figure_components[entity] = None;

        self.free_entities.push(entity);
    }
}

#[cfg(test)]
mod tests {
    use crate::{math::Vector2, utils::Color};

    use super::*;

    #[test]
    fn test_add_initial_entities() {
        let mut scene = Scene::new();

        scene.add_initial_entities(vec![
            ComponentsPayload::new(
                Name::new(String::from("Mario")),
                Transform::default(),
                None,
                Some(Figure::new(Color::RED, Vector2::new(5., 10.))),
            ),
            ComponentsPayload::new(
                Name::new(String::from("Bowser")),
                Transform::default(),
                None,
                Some(Figure::new(Color::RED, Vector2::new(5., 10.))),
            ),
        ]);

        assert_eq!(scene.next_entity, 2);

        assert!(scene.entities[0].is_some());
        assert!(scene.entities[1].is_some());
        assert!(scene.entities[2].is_none());

        assert_eq!(scene.name_components[0].as_ref().unwrap().value(), "Mario");
        assert_eq!(scene.name_components[1].as_ref().unwrap().value(), "Bowser");
        assert!(scene.name_components[2].is_none());
    }

    #[test]
    fn test_add_and_remove_via_spawner() {
        let mut scene = Scene::new();

        scene.add_initial_entities(vec![
            ComponentsPayload::new(
                Name::new(String::from("Mario")),
                Transform::default(),
                None,
                Some(Figure::new(Color::RED, Vector2::new(5., 10.))),
            ),
            ComponentsPayload::new(
                Name::new(String::from("Bowser")),
                Transform::default(),
                None,
                Some(Figure::new(Color::RED, Vector2::new(5., 10.))),
            ),
        ]);

        scene
            .spawner
            .add_entity(ComponentsPayload::from_name(Name::new(String::from(
                "Peach",
            ))));

        scene.add_entities();
        scene.remove_entities();

        assert_eq!(scene.next_entity, 3);
        assert!(scene.entities[2].is_some());

        scene.spawner.remove_entity(1);

        scene.add_entities();
        scene.remove_entities();

        assert_eq!(scene.free_entities.len(), 1);
        assert!(scene.entities[1].is_none());
        assert!(scene.name_components[1].is_none());

        assert!(scene.entities[2].is_some());
        assert!(scene.name_components[2].is_some());

        scene
            .spawner
            .add_entity(ComponentsPayload::from_name(Name::new(String::from(
                "Captain Toad",
            ))));

        scene.add_entities();
        scene.remove_entities();

        assert!(scene.free_entities.is_empty());
        assert_eq!(scene.next_entity, 3);
        assert!(scene.entities[1].is_some());
        assert_eq!(
            scene.name_components[1].as_ref().unwrap().value(),
            "Captain Toad"
        );
    }
}
