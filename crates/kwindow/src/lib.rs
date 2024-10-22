use kutils::{Color, Size};

mod fps_controller;
mod input_processor;
mod renderer;

pub use fps_controller::FpsController;
pub use input_processor::InputProcessor;
pub use renderer::Renderer;

pub struct Window {
    pub fps_controller: FpsController,
    pub renderer: Renderer,
    pub input_processor: InputProcessor,
}

pub fn init(
    title: &str,
    resolution: Size,
    clear_color: Color,
    target_fps: u32,
    min_update_fps: u32,
) -> Window {
    let sdl = sdl2::init().unwrap_or_else(|e| {
        panic!("Failed to initialize SDL2: {}", e);
    });

    Window {
        renderer: Renderer::new(&sdl, &title, resolution, clear_color),
        fps_controller: FpsController::new(
            sdl.timer()
                .unwrap_or_else(|e| panic!("Failed to get SDL2 timer: {}", e)),
            target_fps,
            min_update_fps,
        ),
        input_processor: InputProcessor::new(
            sdl.event_pump()
                .unwrap_or_else(|e| panic!("Failed to get SDL2 event pump: {}", e)),
        ),
    }
}
