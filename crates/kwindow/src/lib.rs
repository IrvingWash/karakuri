use kutils::{Color, Size};
use raylib::RaylibHandle;

mod fps_controller;
mod input_processor;
mod renderer;

pub use fps_controller::FpsController;
pub use input_processor::InputProcessor;
pub use raylib::consts::KeyboardKey;
pub use raylib::RaylibHandle as WindowCtx;
pub use renderer::Renderer;

pub struct Window {
    pub ctx: RaylibHandle,
    pub renderer: Renderer,
    pub fps_controller: FpsController,
    pub input_processor: InputProcessor,
}

pub fn init(title: &str, resolution: Size, clear_color: Color, target_fps: u32) -> Window {
    let (mut rl, thread) = raylib::init()
        .size(resolution.width as i32, resolution.height as i32)
        .title(title)
        .fullscreen()
        .build();

    rl.set_window_focused();

    rl.set_target_fps(target_fps);

    let fps_controller = FpsController::new(target_fps);
    let input_processor = InputProcessor::new();
    let renderer = Renderer::new(thread, clear_color);

    Window {
        ctx: rl,
        renderer,
        fps_controller,
        input_processor,
    }
}
