use kmath::Vector2;
use kphysics::{
    force_generator,
    shapes::{Circle, Shape},
    RigidBody,
};
use raylib::{
    color::Color,
    consts::{KeyboardKey, MouseButton},
    math::Vector2 as RaylibVector2,
    prelude::RaylibDraw,
    RaylibHandle, RaylibThread,
};

const PIXELS_PER_METER: f64 = 50.0;
static mut ANGLE: f64 = 0.0;

#[derive(Debug)]
pub struct App {
    rl: RaylibHandle,
    thread: RaylibThread,
    running: bool,

    rigid_bodies: Vec<RigidBody>,

    push_force: Vector2,

    mouse_position: RaylibVector2,
    is_targeting: bool,
}

impl App {
    pub fn new() -> Self {
        let (handle, thread) = raylib::init().title("Canary").size(1340, 800).build();

        Self {
            running: false,
            rl: handle,
            thread,
            rigid_bodies: Vec::new(),
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

        self.rigid_bodies.push(RigidBody::new(
            Vector2::new(300.0, 300.0),
            5.0,
            Shape::Circle(Circle::new(30.0)),
        ));

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

            let rigid_body = self.rigid_bodies.last_mut().unwrap();

            let impulse_direction = rigid_body
                .position
                .to_subtracted(&mouse_position)
                .to_normalized();

            let impulse_magnitude = rigid_body
                .position
                .to_subtracted(&mouse_position)
                .magnitude()
                * 5.0;

            rigid_body
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

        for rigid_body in &mut self.rigid_bodies {
            rigid_body.apply_force(&force_generator::friction(rigid_body, 100.0));
            rigid_body.apply_force(&force_generator::weight(rigid_body, PIXELS_PER_METER));
            rigid_body.apply_force(&self.push_force);

            rigid_body.integrate(delta_time.into());
        }

        self.keep_in_window();

        self.push_force.reset();
    }

    pub fn render(&mut self) {
        let mut d = self.rl.begin_drawing(&self.thread);

        d.clear_background(Color::GRAY);

        // Visualize force being applied by mouse
        if self.is_targeting {
            let rigid_body = self.rigid_bodies.last_mut().unwrap();

            d.draw_line(
                self.mouse_position.x as i32,
                self.mouse_position.y as i32,
                rigid_body.position.x as i32,
                rigid_body.position.y as i32,
                Color::RED,
            );
        }

        // Draw the rigid_bodies
        for rigid_body in &self.rigid_bodies {
            if rigid_body.shape.is_circle() {
                let radius = rigid_body.shape.circle().unwrap().radius;

                let (angle_cos, angle_sin) = unsafe {
                    ANGLE += 0.01;
                    (ANGLE.cos(), ANGLE.sin())
                };

                d.draw_line(
                    rigid_body.position.x as i32,
                    rigid_body.position.y as i32,
                    (rigid_body.position.x + angle_cos * radius) as i32,
                    (rigid_body.position.y + angle_sin * radius) as i32,
                    Color::WHITE,
                );
                d.draw_circle_lines(
                    rigid_body.position.x as i32,
                    rigid_body.position.y as i32,
                    radius as f32,
                    Color::WHITE,
                );
            }
        }
    }

    fn keep_in_window(&mut self) {
        let width: f64 = self.rl.get_screen_width().into();
        let height: f64 = self.rl.get_screen_height().into();

        for rigid_body in &mut self.rigid_bodies {
            if rigid_body.shape.is_circle() {
                let radius = rigid_body.shape.circle().unwrap().radius;

                if rigid_body.position.x + radius >= width {
                    rigid_body.position.x = width - radius;
                    rigid_body.velocity.x *= -0.9;

                    return;
                }

                if rigid_body.position.x - radius <= 0.0 {
                    rigid_body.position.x = radius;
                    rigid_body.velocity.x *= -0.9;

                    return;
                }

                if rigid_body.position.y + radius >= height {
                    rigid_body.position.y = height - radius;
                    rigid_body.velocity.y *= -0.9;
                }

                if rigid_body.position.y - radius <= 0.0 {
                    rigid_body.position.y = radius;
                    rigid_body.velocity.y *= -0.9;
                }
            }
        }
    }
}
