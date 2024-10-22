use kutils::{Color, Size};
use kwindow::{FpsController, InputProcessor, Renderer};

pub struct Engine {
    renderer: Renderer,
    fps_controller: FpsController,
    input_processor: InputProcessor,
}

impl Engine {
    pub fn new() -> Self {
        let window = kwindow::init("Karakuri", Size::new(800, 600), Color::BLUE, 60, 30);

        Self {
            fps_controller: window.fps_controller,
            input_processor: window.input_processor,
            renderer: window.renderer,
        }
    }

    pub fn play(&mut self) {
        loop {
            // Prepare
            let _delta_time = self.fps_controller.cap_framerate();

            self.input_processor.process();
            let input_result = self.input_processor.result();
            if input_result.should_quit {
                break;
            }

            // Update

            // Render
            self.renderer.start_frame();
            self.renderer.finish_frame();
        }
    }
}
