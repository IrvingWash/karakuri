use kec::{Entity, Registry};
use kmath::Vector2;
use kutils::Size;
use kwindow::AssetStorage;

use crate::components::ComponentPayload;

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

    pub fn create_initial_entities(&mut self, entities: Vec<ComponentPayload>) {
        for entity in entities {
            self.create_entity(entity);
        }
    }

    fn create_entity(&mut self, component_payload: ComponentPayload) {
        self.entities_to_add.push(component_payload);
    }

    pub fn sync(&mut self, registry: &mut Registry, asset_storage: &AssetStorage) -> Vec<Entity> {
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

            if let Some(mut sprite) = bundle.sprite {
                let texture = asset_storage.texture(sprite.texture_name).unwrap(); // TODO: Unwrap

                match &sprite.clip_size {
                    Some(_) => {}
                    None => {
                        sprite.clip_size = Some(Size::new(
                            i64::from(texture.width),
                            i64::from(texture.height),
                        ))
                    }
                }

                match &sprite.rotation_origin {
                    Some(_) => {}
                    None => {
                        sprite.rotation_origin = Some(Vector2::new(
                            sprite.clip_size.unwrap().width as f64 / 2.0, // TODO: Unwrap
                            sprite.clip_size.unwrap().height as f64 / 2.0, // TODO: Unwrap
                        ))
                    }
                }

                registry.add_component(&entity, sprite);
            }

            if let Some(figure) = bundle.figure {
                registry.add_component(&entity, figure);
            }
        }

        entities_to_start
    }
}
