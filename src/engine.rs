use sdl2::Sdl;

use crate::{
    core::{FpsController, InputController, Renderer},
    scene::{ComponentsPayload, Scene},
    utils::{Color, Resolution},
};

pub struct Engine {
    fps_controller: FpsController,
    renderer: Renderer,
    input_controller: InputController,
    scene: Scene,
}

impl Engine {
    pub fn new(
        title: String,
        resolution: Resolution,
        clear_color: Color,
        target_fps: u32,
        min_update_fps: u32,
    ) -> Engine {
        let sdl = Engine::init_sdl();

        Engine {
            scene: Scene::new(),
            renderer: Renderer::new(&sdl, &title, resolution, clear_color),
            fps_controller: FpsController::new(
                sdl.timer()
                    .unwrap_or_else(|e| panic!("Failed to get SDL2 timer: {}", e)),
                target_fps,
                min_update_fps,
            ),
            input_controller: InputController::new(
                sdl.event_pump()
                    .unwrap_or_else(|e| panic!("Failed to get SDL2 event pump: {}", e)),
            ),
        }
    }

    pub fn start(&mut self) {
        self.scene.play(
            &mut self.fps_controller,
            &mut self.renderer,
            &mut self.input_controller,
        );
    }

    pub fn resolution(&self) -> Resolution {
        self.renderer.resolution()
    }

    pub fn add_entity(&mut self, components: ComponentsPayload) {
        self.scene.add_entity(components);
    }

    fn init_sdl() -> Sdl {
        sdl2::init().unwrap_or_else(|e| {
            panic!("Failed to initialize SDL2: {}", e);
        })
    }
}
