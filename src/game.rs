use kec::Registry;
use kwindow::{FpsController, InputProcessor, Renderer, Window};

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
}

impl Game {
    pub fn new(config: GameConfig) -> Self {
        let Window {
            fps_controller,
            renderer,
            input_processor,
        } = kwindow::init_kwindow(
            config.title,
            config.resolution,
            config.clear_color,
            config.target_fps,
            config.min_update_fps,
        );

        Self {
            fps_controller,
            renderer,
            input_processor,
            registry: Registry::new(),
            scene: Scene::new(),
        }
    }

    pub fn set_scene(&mut self, entities: Vec<ComponentPayload>) {
        self.scene
            .create_initial_entities(&mut self.registry, entities);
    }

    pub fn start(&mut self) {
        loop {
            let delta_time = self.fps_controller.cap_framerate();

            // Get input
            let input = self.input_processor.process();
            if input.should_quit {
                break;
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
                        input_result: &input,
                        registry: &self.registry,
                        entity: &entity,
                    });
            }

            // Render
            self.renderer.start_frame();

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

                self.renderer
                    .filled_rectangle(&transform.position, &sprite.size, &sprite.color);
            }

            self.renderer.finish_frame();
        }
    }
}
