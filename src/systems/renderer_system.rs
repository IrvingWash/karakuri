use std::cell::Ref;

use kec::Registry;
use kutils::Size;
use kwindow::{AssetStorage, DrawHandle, Renderer, WindowCtx};

use crate::components::{SpriteComponent, TransformComponent};

use crate::errors::{panic_not_loaded_texture, panic_queried, panic_uninitialized_sprite};

#[derive(Debug)]
pub struct RendererSystem {
    renderer: Renderer,
}

impl RendererSystem {
    pub fn new(renderer: Renderer) -> Self {
        Self { renderer }
    }

    pub fn start_frame<'a>(&self, ctx: &'a mut WindowCtx) -> DrawHandle<'a> {
        self.renderer.start_frame(ctx)
    }

    pub fn finish_frame(&self, handle: DrawHandle) {
        self.renderer.finish_frame(handle);
    }

    pub fn resolution(&self, ctx: &WindowCtx) -> Size {
        self.renderer.resolution(ctx)
    }

    pub fn draw_sprites(
        &self,
        handle: &mut DrawHandle,
        registry: &mut Registry,
        asset_storage: &AssetStorage,
    ) {
        let drawable_entities = registry
            .query()
            .with_component::<TransformComponent>()
            .with_component::<SpriteComponent>()
            .build();

        let mut data: Vec<SpriteDrawData> = Vec::with_capacity(drawable_entities.capacity());

        for entity in drawable_entities {
            data.push(SpriteDrawData {
                transform: registry
                    .get_component::<TransformComponent>(&entity)
                    .unwrap_or_else(|| panic_queried::<TransformComponent>(entity)),
                sprite: registry
                    .get_component::<SpriteComponent>(&entity)
                    .unwrap_or_else(|| panic_queried::<SpriteComponent>(entity)),
            });
        }

        data.sort_by(|a, b| a.sprite.layer.cmp(&b.sprite.layer));

        for SpriteDrawData { transform, sprite } in data {
            let texture = asset_storage
                .texture(sprite.texture_name)
                .unwrap_or_else(|| panic_not_loaded_texture(sprite.texture_name));

            self.renderer.draw_texture(
                handle,
                texture,
                &sprite.clip_position,
                sprite
                    .clip_size
                    .as_ref()
                    .unwrap_or_else(|| panic_uninitialized_sprite("clip_size")),
                &transform.position,
                sprite
                    .clip_size
                    .as_ref()
                    .unwrap_or_else(|| panic_uninitialized_sprite("clip_size")),
                &transform.scale,
                sprite
                    .rotation_origin
                    .as_ref()
                    .unwrap_or_else(|| panic_uninitialized_sprite("rotation_origin")),
                transform.rotation,
                &sprite.tint,
            );
        }
    }
}

struct SpriteDrawData<'a> {
    transform: Ref<'a, TransformComponent>,
    sprite: Ref<'a, SpriteComponent>,
}
