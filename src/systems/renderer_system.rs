use std::cell::Ref;

use kec::Registry;
use kmath::Vector2;
use kutils::Color;
use kwindow::{AssetStorage, DrawHandle, Renderer, WindowCtx};

use crate::components::{
    BoxColliderComponent, CameraComponent, SpriteComponent, TransformComponent,
};

use crate::errors::{
    panic_not_loaded_texture, panic_queried, panic_uninitialized_collider,
    panic_uninitialized_sprite,
};

#[derive(Debug)]
pub struct RendererSystem {
    renderer: Renderer,
}

impl RendererSystem {
    #[inline]
    pub const fn new(renderer: Renderer) -> Self {
        Self { renderer }
    }

    #[inline]
    pub fn start_frame<'a>(&self, ctx: &'a mut WindowCtx) -> DrawHandle<'a> {
        self.renderer.start_frame(ctx)
    }

    #[inline]
    pub fn finish_frame(&self, handle: DrawHandle) {
        self.renderer.finish_frame(handle);
    }

    #[inline]
    pub fn resolution(&self, ctx: &WindowCtx) -> Vector2 {
        self.renderer.resolution(ctx)
    }

    #[inline]
    pub fn draw_fps(&self, handle: &mut DrawHandle, fps: &str, resolution: &Vector2) {
        self.renderer.draw_text(
            handle,
            fps,
            &Vector2::new(resolution.x - 28.0, resolution.y - 28.0),
            14,
            &Color::WHITE,
        );
    }

    #[inline]
    pub fn draw_box_colliders(&self, handle: &mut DrawHandle, registry: &mut Registry) {
        let entities_with_colliders = registry
            .query()
            .with_component::<TransformComponent>()
            .with_component::<BoxColliderComponent>()
            .build();

        for entity in &entities_with_colliders {
            let transform = registry
                .get_component::<TransformComponent>(entity)
                .unwrap_or_else(|| panic_queried::<TransformComponent>(entity));
            let box_collider = registry
                .get_component::<BoxColliderComponent>(entity)
                .unwrap_or_else(|| panic_queried::<BoxColliderComponent>(entity));

            let position = transform.position.to_added(&box_collider.position_offset);
            let halved_position = Vector2::new(
                position.x
                    - box_collider
                        .size
                        .as_ref()
                        .unwrap_or_else(|| panic_uninitialized_collider("size"))
                        .x
                        * transform.scale.x
                        / 2.0,
                position.y
                    - box_collider
                        .size
                        .as_ref()
                        .unwrap_or_else(|| panic_uninitialized_collider("size"))
                        .y
                        * transform.scale.y
                        / 2.0,
            );

            self.renderer.draw_rect(
                handle,
                &halved_position,
                &box_collider
                    .size
                    .as_ref()
                    .unwrap_or_else(|| panic_uninitialized_collider("size"))
                    .to_scaled_by_other(&transform.scale),
                &Color::GREEN,
            );
        }
    }

    #[inline]
    pub fn draw_sprites(
        &self,
        handle: &mut DrawHandle,
        registry: &mut Registry,
        asset_storage: &AssetStorage,
        resolution: &Vector2,
    ) {
        let operator = registry
            .query()
            .with_component::<CameraComponent>()
            .build()
            .first()
            .cloned();

        let (operator_position, zoom) = match operator {
            Some(operator) => {
                let position = registry
                    .get_component::<TransformComponent>(&operator)
                    .unwrap_or_else(|| panic_queried::<TransformComponent>(&operator))
                    .position
                    .clone();

                let camera = registry
                    .get_component::<CameraComponent>(&operator)
                    .unwrap_or_else(|| panic_queried::<CameraComponent>(&operator))
                    .zoom;

                (position, camera)
            }
            None => (resolution.to_divided(2.0), CameraComponent::default().zoom),
        };

        let drawable_entities = registry
            .query()
            .with_component::<TransformComponent>()
            .with_component::<SpriteComponent>()
            .build();

        let mut data: Vec<SpriteDrawData> = Vec::with_capacity(drawable_entities.capacity());

        for entity in &drawable_entities {
            data.push(SpriteDrawData {
                transform: registry
                    .get_component::<TransformComponent>(entity)
                    .unwrap_or_else(|| panic_queried::<TransformComponent>(entity)),
                sprite: registry
                    .get_component::<SpriteComponent>(entity)
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
                &transform.position.to_scaled(zoom).to_subtracted(
                    &operator_position
                        .to_scaled(zoom)
                        .to_subtracted(&resolution.to_divided(2.0)),
                ),
                &sprite
                    .clip_size
                    .as_ref()
                    .unwrap_or_else(|| panic_uninitialized_sprite("clip_size"))
                    .to_scaled_by_other(&transform.scale)
                    .to_scaled(zoom),
                &sprite
                    .origin
                    .as_ref()
                    .unwrap_or_else(|| panic_uninitialized_sprite("rotation_origin"))
                    .to_scaled_by_other(&transform.scale)
                    .to_scaled(zoom),
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
