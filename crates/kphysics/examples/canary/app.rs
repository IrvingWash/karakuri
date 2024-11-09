use kmath::Vector2;
use kphysics::Particle;
use raylib::{
    color::Color, math::Vector2 as RaylibVector2, prelude::RaylibDraw, RaylibHandle, RaylibThread,
};

const PIXELS_PER_METER: f64 = 50.0;

#[derive(Debug)]
pub struct App {
    rl: RaylibHandle,
    thread: RaylibThread,
    running: bool,

    particles: Vec<Particle>,
}

impl App {
    pub fn new() -> Self {
        let (handle, thread) = raylib::init().title("Canary").build();

        Self {
            running: false,
            rl: handle,
            thread,
            particles: Vec::new(),
        }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn setup(&mut self) {
        self.rl.set_target_fps(60);
        self.running = true;

        self.particles.push(Particle::new(
            Vector2::new(50.0, 100.0),
            Vector2::ZERO,
            0.5,
            2.0,
        ));

        self.particles.push(Particle::new(
            Vector2::new(50.0, 115.0),
            Vector2::ZERO,
            1.0,
            4.0,
        ));

        self.particles.push(Particle::new(
            Vector2::new(50.0, 130.0),
            Vector2::ZERO,
            2.0,
            8.0,
        ));
    }

    pub fn input(&mut self) {
        self.running = !self.rl.window_should_close();
    }

    pub fn update(&mut self) {
        let delta_time = self.rl.get_frame_time();

        let wind_force = Vector2::new(0.2 * PIXELS_PER_METER, 0.0);
        let weight_force = Vector2::new(0.0, 9.8 * PIXELS_PER_METER);

        for particle in &mut self.particles {
            particle.apply_force(&wind_force);
            particle.apply_force(&weight_force);

            particle.integrate(delta_time.into());
        }

        self.keep_in_window();
    }

    pub fn render(&mut self) {
        let mut d = self.rl.begin_drawing(&self.thread);

        d.clear_background(Color::GRAY);

        for particle in &mut self.particles {
            d.draw_circle_v(
                vector2_to_raylib(&particle.position),
                particle.radius as f32,
                Color::WHITE,
            );
        }
    }

    fn keep_in_window(&mut self) {
        let width: f64 = self.rl.get_screen_width().into();
        let height: f64 = self.rl.get_screen_height().into();

        for particle in &mut self.particles {
            if particle.position.x + particle.radius > width {
                particle.position.x = width - particle.radius;
                particle.velocity.x *= -0.9;

                return;
            }

            if particle.position.x - particle.radius < 0.0 {
                particle.position.x = particle.radius;
                particle.velocity.x *= -0.9;

                return;
            }

            if particle.position.y + particle.radius > height {
                particle.position.y = height - particle.radius;
                particle.velocity.y *= -0.9;
            }

            if particle.position.y - particle.radius < 0.0 {
                particle.position.y = particle.radius;
                particle.velocity.y *= -0.9;
            }
        }
    }
}

fn vector2_to_raylib(vector2: &Vector2) -> RaylibVector2 {
    RaylibVector2 {
        x: vector2.x as f32,
        y: vector2.y as f32,
    }
}
