use kmath::Vector2;
use kphysics::{particle_force_generator, Particle};
use raylib::{
    color::Color,
    consts::{KeyboardKey, MouseButton},
    math::Vector2 as RaylibVector2,
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

    mouse_position: RaylibVector2,
    is_targeting: bool,
}

impl App {
    pub fn new() -> Self {
        let (handle, thread) = raylib::init().title("Spring").size(1340, 800).build();

        Self {
            running: false,
            rl: handle,
            thread,
            particles: Vec::new(),
            mouse_position: RaylibVector2::zero(),
            is_targeting: false,
            push_force: Vector2::ZERO,
        }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn setup(&mut self) {
        self.rl.set_target_fps(60);

        // Anchor
        self.particles
            .push(Particle::new(Vector2::new(600.0, 10.0), 0.0, 5.0));

        // Bobs
        for i in 1..=15 {
            self.particles.push(Particle::new(
                Vector2::new(600.0, i as f64 * 15.0),
                2.0,
                5.0,
            ))
        }

        self.running = true;
    }

    pub fn input(&mut self) {
        self.running = !self.rl.window_should_close();

        if self.rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
            self.is_targeting = true;

            self.mouse_position = self.rl.get_mouse_position();
        }

        if self
            .rl
            .is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT)
        {
            self.is_targeting = false;

            let mouse_position =
                Vector2::new(self.mouse_position.x as f64, self.mouse_position.y as f64);

            let particle = self.particles.last_mut().unwrap();

            let impulse_direction = particle
                .position
                .to_subtracted(&mouse_position)
                .to_normalized();

            let impulse_magnitude =
                particle.position.to_subtracted(&mouse_position).magnitude() * 5.0;

            particle
                .velocity
                .set(&impulse_direction.to_scaled(impulse_magnitude));
        }

        if self.rl.is_key_down(KeyboardKey::KEY_LEFT) {
            self.push_force
                .add(&Vector2::new(-50.0 * PIXELS_PER_METER, 0.0));
        }
        if self.rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            self.push_force
                .add(&Vector2::new(50.0 * PIXELS_PER_METER, 0.0));
        }
    }

    pub fn update(&mut self) {
        let delta_time = self.rl.get_frame_time();

        for i in 1..self.particles.len() {
            let current = &self.particles[i];
            let previous = &self.particles[i - 1];

            let spring_force = particle_force_generator::spring(&current, &previous, 15.0, 300.0);

            self.particles[i].apply_force(&spring_force);
            self.particles[i - 1].apply_force(&spring_force.to_scaled(-1.0));
        }

        for particle in &mut self.particles {
            particle.apply_force(&particle_force_generator::drag(particle, 0.002));
            particle.apply_force(&particle_force_generator::weight(
                particle,
                PIXELS_PER_METER,
            ));
            particle.apply_force(&self.push_force);

            particle.integrate(delta_time.into());
        }

        self.keep_in_window();

        self.push_force.reset();
    }

    pub fn render(&mut self) {
        let mut d = self.rl.begin_drawing(&self.thread);

        d.clear_background(Color::GRAY);

        // Visualize force being applied by mouse
        if self.is_targeting {
            let particle = self.particles.last_mut().unwrap();

            d.draw_line(
                self.mouse_position.x as i32,
                self.mouse_position.y as i32,
                particle.position.x as i32,
                particle.position.y as i32,
                Color::RED,
            );
        }

        // Draw spring between particles
        for i in 1..self.particles.len() {
            d.draw_line_ex(
                vector2_to_raylib(&self.particles[i - 1].position),
                vector2_to_raylib(&self.particles[i].position),
                15.0,
                Color::GREEN,
            );
        }

        // Draw the particles
        for particle in &self.particles {
            d.draw_circle_v(
                vector2_to_raylib(&particle.position),
                particle.radius as f32,
                Color::WHEAT,
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
