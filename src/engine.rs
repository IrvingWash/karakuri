use kutils::{Color, Size};
use kwindow::{FpsController, InputProcessor, Renderer};

use crate::{
    components::{Behavior, Sprite, Transform},
    Scene,
};

pub struct Engine {
    renderer: Renderer,
    fps_controller: FpsController,
    input_processor: InputProcessor,
    scene: Scene,
}

impl Engine {
    pub fn new() -> Self {
        let window = kwindow::init("Karakuri", Size::new(800, 600), Color::BLUE, 60, 30);

        Self {
            fps_controller: window.fps_controller,
            input_processor: window.input_processor,
            renderer: window.renderer,
            scene: Scene::new(),
        }
    }

    pub fn scene(&mut self) -> &mut Scene {
        &mut self.scene
    }

    pub fn play(&mut self) {
        loop {
            // Prepare
            self.scene.sync();

            let delta_time = self.fps_controller.cap_framerate();

            self.input_processor.process();
            let input_result = self.input_processor.result();
            if input_result.should_quit {
                break;
            }

            // Update
            if let Some(behaviors) = self
                .scene
                .world
                .get_component_vec_mut::<Box<dyn Behavior>>()
            {
                for mut b in behaviors {
                    b.update(delta_time);
                }
            }

            // Render
            self.renderer.start_frame();
            if let Some(transforms) = self.scene.world.get_component_vec::<Transform>() {
                if let Some(sprites) = self.scene.world.get_component_vec::<Sprite>() {
                    for entity in self.scene.world.entities() {
                        let transform = &transforms[*entity];
                        let sprite = &sprites[*entity];

                        self.renderer.filled_rectangle(
                            &transform.position,
                            &sprite.size,
                            &sprite.color,
                        );
                    }
                };
            }

            self.renderer.finish_frame();
        }
    }
}
