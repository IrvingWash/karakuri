use kmath::Vector2;
use kphysics::Particle;
use raylib::{
    color::Color, math::Vector2 as RaylibVector2, prelude::RaylibDraw, RaylibHandle, RaylibThread,
};

#[derive(Debug)]
pub struct App {
    rl: RaylibHandle,
    thread: RaylibThread,
    running: bool,

    particle: Option<Particle>,
}

impl App {
    pub fn new() -> Self {
        let (handle, thread) = raylib::init().title("Canary").build();

        Self {
            running: false,
            rl: handle,
            thread,
            particle: None,
        }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn setup(&mut self) {
        self.rl.toggle_borderless_windowed();
        self.rl.set_target_fps(60);
        self.running = true;

        self.particle = Some(Particle::new(Vector2::new(50.0, 100.0), Vector2::ZERO, 1.0));
    }

    pub fn input(&mut self) {
        self.running = !self.rl.window_should_close();
    }

    pub fn update(&mut self) {
        let particle = self.particle.as_mut().unwrap();

        particle.velocity = Vector2::new(100.0, 30.0);
        particle
            .position
            .add(&particle.velocity.to_scaled(self.rl.get_frame_time().into()));
    }

    pub fn render(&mut self) {
        let mut d = self.rl.begin_drawing(&self.thread);

        d.clear_background(Color::GRAY);

        d.draw_circle_v(
            vector2_to_raylib(&self.particle.as_ref().unwrap().position),
            4.0,
            Color::WHITE,
        );
    }
}

fn vector2_to_raylib(vector2: &Vector2) -> RaylibVector2 {
    RaylibVector2 {
        x: vector2.x as f32,
        y: vector2.y as f32,
    }
}
