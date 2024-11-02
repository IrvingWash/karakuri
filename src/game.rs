use kec::Registry;
use kutils::Size;
use kwindow::{AssetStorage, FpsController, InputProcessor, Window, WindowCtx};

use crate::{
    components::{BehaviorComponent, ComponentPayload, Ctx},
    errors::panic_queried,
    renderer_adapter::RendererAdapter,
    GameConfig, Scene,
};

pub struct Game {
    fps_controller: FpsController,
    renderer_adapter: RendererAdapter,
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
            renderer_adapter: RendererAdapter::new(renderer),
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
            let mut handle = self.renderer_adapter.start_frame(&mut self.ctx);

            self.renderer_adapter
                .draw_figures(&mut handle, &mut self.registry);

            self.renderer_adapter.draw_sprites(
                &mut handle,
                &mut self.registry,
                &self.asset_storage,
            );

            self.renderer_adapter.finish_frame(handle);
        }
    }

    pub fn resolution(&self) -> Size {
        self.renderer_adapter.resolution(&self.ctx)
    }
}
