use kec::{Entity, Registry};
use kmath::Vector2;
use kutils::Size;
use kwindow::AssetStorage;

use crate::{
    components::ComponentPayload,
    errors::{panic_not_loaded_texture, panic_uninitialized_sprite},
};

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

    pub fn sync(
        &mut self,
        registry: &mut Registry,
        asset_storage: &AssetStorage,
        time: f64,
    ) -> Vec<Entity> {
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
                let texture = asset_storage
                    .texture(sprite.texture_name)
                    .unwrap_or_else(|| panic_not_loaded_texture(sprite.texture_name));

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
                            sprite
                                .clip_size
                                .unwrap_or_else(|| panic_uninitialized_sprite("clip_size"))
                                .width as f64
                                / 2.0,
                            sprite
                                .clip_size
                                .unwrap_or_else(|| panic_uninitialized_sprite("clip_size"))
                                .height as f64
                                / 2.0,
                        ))
                    }
                }

                registry.add_component(&entity, sprite);
            }

            if let Some(figure) = bundle.figure {
                registry.add_component(&entity, figure);
            }

            if let Some(mut animation) = bundle.animation {
                animation.start_time = time;

                registry.add_component(&entity, animation);
            }

            if let Some(rigid_body) = bundle.rigid_body {
                registry.add_component(&entity, rigid_body);
            }

            if let Some(box_collider) = bundle.box_collider {
                registry.add_component(&entity, box_collider);
            }
        }

        entities_to_start
    }
}
