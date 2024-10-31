use kec::Registry;
use kutils::Size;
use kwindow::{FpsController, InputProcessor, Renderer, Window, WindowCtx};

use crate::{
    components::{BehaviorComponent, ComponentPayload, Ctx, SpriteComponent, TransformComponent},
    GameConfig, Scene,
};

pub struct Game {
    fps_controller: FpsController,
    renderer: Renderer,
    input_processor: InputProcessor,
    registry: Registry,
    scene: Scene,
    ctx: WindowCtx,
}

impl Game {
    pub fn new(config: GameConfig) -> Self {
        let Window {
            fps_controller,
            renderer,
            input_processor,
            ctx,
        } = kwindow::init(
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
        }
    }

    pub fn set_scene(&mut self, entities: Vec<ComponentPayload>) {
        self.scene.create_initial_entities(entities);
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
                    .unwrap()
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
                    .unwrap()
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

            let renderable_entities = self
                .registry
                .query()
                .with_component::<TransformComponent>()
                .with_component::<SpriteComponent>()
                .build();

            for entity in renderable_entities {
                let transform = self
                    .registry
                    .get_component::<TransformComponent>(&entity)
                    .unwrap();
                let sprite = self
                    .registry
                    .get_component::<SpriteComponent>(&entity)
                    .unwrap();

                handle = self.renderer.draw_rect(
                    handle,
                    &transform.position,
                    &sprite.size,
                    &sprite.color,
                );
            }

            self.renderer.finish_frame(handle);
        }
    }

    pub fn resolution(&self) -> Size {
        self.renderer.resolution(&self.ctx)
    }
}
