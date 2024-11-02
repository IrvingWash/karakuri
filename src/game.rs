use kec::Registry;
use kutils::Size;
use kwindow::{AssetStorage, FpsController, InputProcessor, Window, WindowCtx};

use crate::{
    adapters::InputProcessorAdapter,
    components::{BehaviorComponent, ComponentPayload, Ctx},
    errors::panic_queried,
    systems::{AnimatorSystem, RendererSystem},
    GameConfig, Scene,
};

pub struct Game {
    fps_controller: FpsController,
    input_processor: InputProcessor,
    registry: Registry,
    scene: Scene,
    ctx: WindowCtx,
    asset_storage: AssetStorage,
    renderer: RendererSystem,
    animator: AnimatorSystem,
}

impl Game {
    pub fn new(config: &GameConfig) -> Self {
        let Window {
            fps_controller,
            renderer,
            input_processor,
            ctx,
            asset_storage,
        } = Window::new(
            config.title,
            config.resolution,
            &config.clear_color,
            config.target_fps,
        );

        Self {
            fps_controller,
            input_processor,
            registry: Registry::new(),
            scene: Scene::new(),
            ctx,
            asset_storage,
            renderer: RendererSystem::new(renderer),
            animator: AnimatorSystem::new(),
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
            let time = self.fps_controller.time(&self.ctx);
            let delta_time = self.fps_controller.delta_time(&self.ctx);

            // Get input
            let input_processor_adapter =
                InputProcessorAdapter::new(&self.input_processor, &self.ctx);

            if input_processor_adapter.should_close() {
                break;
            }

            // Start new entities
            let entities_to_start = self
                .scene
                .sync(&mut self.registry, &self.asset_storage, time);

            for entity in entities_to_start {
                self.registry
                    .get_component_mut::<Box<dyn BehaviorComponent>>(&entity)
                    .unwrap_or_else(|| panic_queried::<Box<dyn BehaviorComponent>>(entity))
                    .start(Ctx {
                        entity: &entity,
                        delta_time,
                        registry: &self.registry,
                        input_processor: &input_processor_adapter,
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
                        input_processor: &input_processor_adapter,
                    });
            }

            self.animator.animate(&mut self.registry, time);

            self.render();
        }
    }

    pub fn resolution(&self) -> Size {
        self.renderer.resolution(&self.ctx)
    }

    fn render(&mut self) {
        let mut handle = self.renderer.start_frame(&mut self.ctx);

        self.renderer.draw_figures(&mut handle, &mut self.registry);

        self.renderer
            .draw_sprites(&mut handle, &mut self.registry, &self.asset_storage);

        self.renderer.finish_frame(handle);
    }
}
