use core::panic;
use std::mem;

use kec::{Entity, Registry};
use kmath::Vector2;
use kwindow::AssetStorage;

use crate::{
    components::{BoxColliderComponent, CameraComponent, ComponentPayload, SpriteComponent},
    errors::{panic_not_loaded_texture, panic_uninitialized_sprite},
};

use super::Spawner;

#[derive(Debug, Default)]
pub struct Scene {
    entities_to_remove: Vec<Entity>,
    spawner: Spawner,
}

impl Scene {
    #[inline]
    pub const fn new() -> Self {
        Self {
            entities_to_remove: Vec::new(),
            spawner: Spawner::new(),
        }
    }

    #[inline]
    pub fn create_initial_entities(&mut self, mut entities: Vec<ComponentPayload>) {
        self.spawner.entities_to_add.append(&mut entities);
    }

    #[inline]
    pub fn sync(
        &mut self,
        registry: &mut Registry,
        asset_storage: &AssetStorage,
    ) -> (Vec<Entity>, Vec<Entity>) {
        // Delete intersection of entities to destroy and entities to remove from entities to destroy
        self.spawner.entities_to_destroy = mem::take(&mut self.spawner.entities_to_destroy)
            .into_iter()
            .filter(|e| !self.entities_to_remove.contains(e))
            .collect();

        // Actually delete already destroyed entities
        for entity in mem::take(&mut self.entities_to_remove) {
            registry.remove_entity(&entity);
        }

        // Create entities
        let mut entities_to_start: Vec<Entity> = Vec::new();

        let entities_to_add = mem::take(&mut self.spawner.entities_to_add);
        for bundle in entities_to_add {
            let entity = registry.create_entity();

            let mut sprite_clone: Option<SpriteComponent> = None;

            if let Some(transform) = bundle.transform {
                registry.add_component(&entity, transform);
            }

            if let Some(behavior) = bundle.behavior {
                registry.add_component(&entity, behavior);
                entities_to_start.push(entity.clone());
            }

            if let Some(tab) = bundle.tag {
                registry.add_component(&entity, tab);
            }

            if let Some(sprite) = bundle.sprite {
                let populated_sprite = self.prepare_sprite_component(sprite, asset_storage);

                sprite_clone = Some(populated_sprite.clone());

                registry.add_component(
                    &entity,
                    self.prepare_sprite_component(populated_sprite, asset_storage),
                );
            }

            if let Some(animation) = bundle.animation_controller {
                registry.add_component(&entity, animation);
            }

            if let Some(rigid_body) = bundle.rigid_body {
                registry.add_component(&entity, rigid_body);
            }

            if let Some(box_collider) = bundle.box_collider {
                registry.add_component(
                    &entity,
                    self.prepare_box_collider_component(box_collider, sprite_clone),
                );
            }

            if let Some(camera) = bundle.camera {
                if registry.has::<CameraComponent>() {
                    panic!("Only one camera is allowed on the scene.");
                }

                registry.add_component(&entity, camera);
            }
        }

        (
            entities_to_start,
            mem::take(&mut self.spawner.entities_to_destroy),
        )
    }

    #[inline]
    pub fn set_entities_to_remove(&mut self, entities: Vec<Entity>) {
        self.entities_to_remove = entities;
    }

    #[inline]
    pub fn spawner(&mut self) -> &mut Spawner {
        &mut self.spawner
    }

    fn prepare_box_collider_component(
        &self,
        mut box_collider: BoxColliderComponent,
        sprite: Option<SpriteComponent>,
    ) -> BoxColliderComponent {
        match box_collider.size {
            Some(_) => {}
            None => match sprite {
                None => box_collider.size = Some(Vector2::new(0.0, 0.0)),
                Some(sprite) => {
                    box_collider.size = Some(
                        sprite
                            .clip_size
                            .unwrap_or_else(|| panic_uninitialized_sprite("clip_size")),
                    )
                }
            },
        }

        box_collider
    }

    fn prepare_sprite_component(
        &self,
        mut sprite: SpriteComponent,
        asset_storage: &AssetStorage,
    ) -> SpriteComponent {
        let texture = asset_storage
            .texture(sprite.texture_name)
            .unwrap_or_else(|| panic_not_loaded_texture(sprite.texture_name));

        match &sprite.clip_size {
            Some(_) => {}
            None => {
                sprite.clip_size = Some(Vector2::new(
                    f64::from(texture.width),
                    f64::from(texture.height),
                ))
            }
        }

        match &sprite.origin {
            Some(_) => {}
            None => {
                sprite.origin = Some(Vector2::new(
                    sprite
                        .clip_size
                        .as_ref()
                        .unwrap_or_else(|| panic_uninitialized_sprite("clip_size"))
                        .x
                        / 2.0,
                    sprite
                        .clip_size
                        .as_ref()
                        .unwrap_or_else(|| panic_uninitialized_sprite("clip_size"))
                        .y
                        / 2.0,
                ))
            }
        }

        sprite
    }
}
