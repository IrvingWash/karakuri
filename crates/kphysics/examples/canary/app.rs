use kmath::Vector2;
use kphysics::{particle_force_generator, Particle};
use raylib::{
    color::Color,
    consts::KeyboardKey,
    math::{Rectangle, Vector2 as RaylibVector2},
    prelude::RaylibDraw,
    RaylibHandle, RaylibThread,
};

const PIXELS_PER_METER: f64 = 50.0;

#[derive(Debug)]
pub struct App {
    rl: RaylibHandle,
    thread: RaylibThread,
    running: bool,

    particles: Vec<Particle>,
    push_force: Vector2,

    liquid: Rectangle,
}

impl App {
    pub fn new() -> Self {
        let (handle, thread) = raylib::init().title("Canary").build();

        Self {
            running: false,
            rl: handle,
            thread,
            particles: Vec::new(),
            push_force: Vector2::ZERO,
            liquid: Rectangle::default(),
        }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn setup(&mut self) {
        self.rl.set_target_fps(60);
        self.running = true;

        self.liquid.x = 0.0;
        self.liquid.y = (self.rl.get_screen_height() / 2) as f32;
        self.liquid.width = self.rl.get_screen_width() as f32;
        self.liquid.height = (self.rl.get_screen_height() / 2) as f32;

        self.particles.push(Particle::new(
            Vector2::new(100.0, 100.0),
            Vector2::ZERO,
            1.0,
            4.0,
        ));
    }

    pub fn input(&mut self) {
        self.running = !self.rl.window_should_close();

        if self.rl.is_key_down(KeyboardKey::KEY_UP) {
            self.push_force
                .add(&Vector2::new(0.0, -50.0 * PIXELS_PER_METER));
        }
        if self.rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            self.push_force
                .add(&Vector2::new(50.0 * PIXELS_PER_METER, 0.0));
        }
        if self.rl.is_key_down(KeyboardKey::KEY_DOWN) {
            self.push_force
                .add(&Vector2::new(0.0, 50.0 * PIXELS_PER_METER));
        }
        if self.rl.is_key_down(KeyboardKey::KEY_LEFT) {
            self.push_force
                .add(&Vector2::new(-50.0 * PIXELS_PER_METER, 0.0));
        }
    }

    pub fn update(&mut self) {
        let delta_time = self.rl.get_frame_time();

        for particle in &mut self.particles {
            let weight_force = particle_force_generator::weight(particle, PIXELS_PER_METER);
            particle.apply_force(&weight_force);

            if particle.position.y >= self.liquid.y.into() {
                let drag_force = particle_force_generator::drag(particle, 0.01);
                particle.apply_force(&drag_force);
            }

            particle.apply_force(&self.push_force);

            particle.integrate(delta_time.into());
        }

        self.keep_in_window();

        self.push_force.reset();
    }

    pub fn render(&mut self) {
        let mut d = self.rl.begin_drawing(&self.thread);

        d.clear_background(Color::GRAY);

        d.draw_rectangle_v(
            RaylibVector2::new(self.liquid.x, self.liquid.y),
            RaylibVector2::new(self.liquid.width, self.liquid.height),
            Color::BLUE,
        );

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
            if particle.position.x + particle.radius >= width {
                particle.position.x = width - particle.radius;
                particle.velocity.x *= -0.9;

                return;
            }

            if particle.position.x - particle.radius <= 0.0 {
                particle.position.x = particle.radius;
                particle.velocity.x *= -0.9;

                return;
            }

            if particle.position.y + particle.radius >= height {
                particle.position.y = height - particle.radius;
                particle.velocity.y *= -0.9;
            }

            if particle.position.y - particle.radius <= 0.0 {
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
