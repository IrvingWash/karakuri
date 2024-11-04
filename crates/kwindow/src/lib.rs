use kutils::{Color, Size};
use raylib::RaylibHandle;

mod asset_storage;
mod fps_controller;
mod input_processor;
mod renderer;
mod timer;

pub use asset_storage::AssetStorage;
pub use fps_controller::FpsController;
pub use input_processor::InputProcessor;
pub use raylib::consts::KeyboardKey;
pub use raylib::prelude::RaylibDrawHandle as DrawHandle;
pub use raylib::texture::Texture2D;
pub use raylib::RaylibHandle as WindowCtx;
pub use renderer::Renderer;
pub use timer::Timer;

pub struct Window {
    pub ctx: RaylibHandle,
    pub renderer: Renderer,
    pub fps_controller: FpsController,
    pub input_processor: InputProcessor,
    pub asset_storage: AssetStorage,
}

impl Window {
    pub fn new(title: &str, resolution: Size, clear_color: &Color, target_fps: u32) -> Self {
        let (mut rl, thread) = raylib::init()
            .size(resolution.width as i32, resolution.height as i32)
            .title(title)
            // .fullscreen()
            .build();

        rl.set_window_focused();

        rl.set_target_fps(target_fps);

        let fps_controller = FpsController::new();
        let input_processor = InputProcessor::new();
        let renderer = Renderer::new(thread.clone(), clear_color);
        let asset_storage = AssetStorage::new(thread);

        Window {
            ctx: rl,
            renderer,
            fps_controller,
            input_processor,
            asset_storage,
        }
    }
}
