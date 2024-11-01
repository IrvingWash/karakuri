use std::any::type_name;

use kec::{Entity, Registry};
use kutils::Size;
use kwindow::{AssetStorage, FpsController, InputProcessor, Renderer, Window, WindowCtx};

use crate::{
    components::{
        BehaviorComponent, ComponentPayload, Ctx, FigureComponent, SpriteComponent,
        TransformComponent,
    },
    GameConfig, Scene,
};

pub struct Game {
    fps_controller: FpsController,
    renderer: Renderer,
    input_processor: InputProcessor,
    registry: Registry,
    scene: Scene,
    ctx: WindowCtx,
    asset_storage: AssetStorage,
}

impl Game {
    pub fn new(config: GameConfig) -> Self {
        let Window {
            fps_controller,
            renderer,
            input_processor,
            ctx,
            asset_storage,
        } = Window::new(
            config.title,
            config.resolution,
            config.clear_color,
            config.target_fps,
        );

        Self {
            fps_controller,
            renderer,
            input_processor,
            registry: Registry::new(),
            scene: Scene::new(),
            ctx,
            asset_storage,
        }
    }

    pub fn set_scene(&mut self, entities: Vec<ComponentPayload>) {
        self.scene.create_initial_entities(entities);
    }

    pub fn add_texture(&mut self, name: &'static str, path: &'static str) -> Result<(), String> {
        self.asset_storage.add_texture(name, path, &mut self.ctx)
    }

    pub fn start(&mut self) {
        loop {
            let delta_time = self.fps_controller.delta_time(&self.ctx);

            // Get input
            if self.input_processor.should_close(&self.ctx) {
                break;
            }

            // Start new entities
            let entities_to_start = self.scene.sync(&mut self.registry);

            for entity in entities_to_start {
                self.registry
                    .get_component_mut::<Box<dyn BehaviorComponent>>(&entity)
                    .unwrap_or_else(|| panic_queried::<Box<dyn BehaviorComponent>>(entity))
                    .start(Ctx {
                        entity: &entity,
                        delta_time,
                        registry: &self.registry,
                        input_processor: &self.input_processor,
                        window_ctx: &self.ctx,
                    });
            }

            // Update
            let updateable_entities = self
                .registry
                .query()
                .with_component::<Box<dyn BehaviorComponent>>()
                .build();

            for entity in updateable_entities {
                self.registry
                    .get_component_mut::<Box<dyn BehaviorComponent>>(&entity)
                    .unwrap_or_else(|| panic_queried::<Box<dyn BehaviorComponent>>(entity))
                    .update(Ctx {
                        delta_time,
                        registry: &self.registry,
                        entity: &entity,
                        input_processor: &self.input_processor,
                        window_ctx: &self.ctx,
                    });
            }

            // Render
            let mut handle = self.renderer.start_frame(&mut self.ctx);

            let entities_with_figures = self
                .registry
                .query()
                .with_component::<TransformComponent>()
                .with_component::<FigureComponent>()
                .build();

            for entity in entities_with_figures {
                let transform = self
                    .registry
                    .get_component::<TransformComponent>(&entity)
                    .unwrap_or_else(|| panic_queried::<TransformComponent>(entity));
                let figure = self
                    .registry
                    .get_component::<FigureComponent>(&entity)
                    .unwrap_or_else(|| panic_queried::<FigureComponent>(entity));

                self.renderer.draw_rect(
                    &mut handle,
                    &transform.position,
                    &figure.size,
                    &figure.color,
                );
            }

            let entities_with_sprites = self
                .registry
                .query()
                .with_component::<TransformComponent>()
                .with_component::<SpriteComponent>()
                .build();

            for entity in entities_with_sprites {
                let transform = self
                    .registry
                    .get_component::<TransformComponent>(&entity)
                    .unwrap_or_else(|| panic_queried::<TransformComponent>(entity));
                let sprite = self
                    .registry
                    .get_component::<SpriteComponent>(&entity)
                    .unwrap_or_else(|| panic_queried::<SpriteComponent>(entity));

                if let Some(texture) = self.asset_storage.texture(sprite.texture_name) {
                    self.renderer.draw_texture(
                        &mut handle,
                        texture,
                        &sprite.clip_position,
                        &sprite.clip_size,
                        &transform.position,
                        &transform.scale,
                        None,
                        transform.rotation,
                        None,
                    );
                }
            }

            self.renderer.finish_frame(handle);
        }
    }

    pub fn resolution(&self) -> Size {
        self.renderer.resolution(&self.ctx)
    }
}

fn panic_queried<T>(entity: Entity) -> ! {
    panic!(
        "Entity {} didn't have {}, though was queried for it.",
        entity.id(),
        type_name::<T>()
    )
}
