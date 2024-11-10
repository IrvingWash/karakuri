use kmath::Vector2;
use kphysics::{
    force_generator,
    shapes::{Rectangle, Shape},
    RigidBody,
};
use raylib::{
    color::Color, consts::KeyboardKey, math::Vector2 as RaylibVector2, prelude::RaylibDraw,
    RaylibHandle, RaylibThread,
};

const PIXELS_PER_METER: f64 = 50.0;

#[derive(Debug)]
pub struct App {
    rl: RaylibHandle,
    thread: RaylibThread,
    running: bool,

    rigid_bodies: Vec<RigidBody>,

    push_force: Vector2,
}

impl App {
    pub fn new() -> Self {
        let (handle, thread) = raylib::init().title("Canary").size(1340, 800).build();

        Self {
            running: false,
            rl: handle,
            thread,
            rigid_bodies: Vec::new(),
            push_force: Vector2::ZERO,
        }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn setup(&mut self) {
        self.rl.set_target_fps(60);

        let screen_width = self.rl.get_screen_width();
        let screen_height = self.rl.get_screen_height();

        self.rigid_bodies.push(RigidBody::new(
            Vector2::new((screen_width / 2) as f64, (screen_height / 2) as f64),
            1.0,
            Shape::Rectangle(Rectangle::new(100.0, 100.0)),
        ));

        self.running = true;
    }

    pub fn input(&mut self) {
        self.running = !self.rl.window_should_close();

        if self.rl.is_key_down(KeyboardKey::KEY_LEFT) {
            self.push_force
                .add(&Vector2::new(-50.0 * PIXELS_PER_METER, 0.0));
        }
        if self.rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            self.push_force
                .add(&Vector2::new(50.0 * PIXELS_PER_METER, 0.0));
        }
        if self.rl.is_key_down(KeyboardKey::KEY_UP) {
            self.push_force
                .add(&Vector2::new(0.0, -50.0 * PIXELS_PER_METER));
        }
        if self.rl.is_key_down(KeyboardKey::KEY_DOWN) {
            self.push_force
                .add(&Vector2::new(0.0, 50.0 * PIXELS_PER_METER));
        }
    }

    pub fn update(&mut self) {
        let delta_time = self.rl.get_frame_time();

        for rigid_body in &mut self.rigid_bodies {
            rigid_body.apply_force(&force_generator::friction(rigid_body, 100.0));
            rigid_body.apply_force(&self.push_force);
            rigid_body.apply_torque(200.0);

            rigid_body.update(delta_time.into());
        }

        self.keep_in_window();

        self.push_force.reset();
    }

    pub fn render(&mut self) {
        let mut d = self.rl.begin_drawing(&self.thread);

        d.clear_background(Color::GRAY);

        for rigid_body in &self.rigid_bodies {
            // Draw circular rigid_bodies
            if rigid_body.shape.is_circle() {
                let radius = rigid_body.shape.circle().unwrap().radius;

                d.draw_line(
                    rigid_body.position.x as i32,
                    rigid_body.position.y as i32,
                    (rigid_body.position.x + rigid_body.rotation.cos() * radius) as i32,
                    (rigid_body.position.y + rigid_body.rotation.sin() * radius) as i32,
                    Color::WHITE,
                );
                d.draw_circle_lines(
                    rigid_body.position.x as i32,
                    rigid_body.position.y as i32,
                    radius as f32,
                    Color::WHITE,
                );
            }

            // Draw rectangular rigid bodies
            if let Some(rectangle) = rigid_body.shape.rectangle() {
                for i in 0..rectangle.world_vertices.len() {
                    let curr = i;
                    let next = (i + 1) % rectangle.world_vertices.len();

                    d.draw_line_ex(
                        vector2_to_raylib(&rectangle.world_vertices[curr]),
                        vector2_to_raylib(&rectangle.world_vertices[next]),
                        1.0,
                        Color::WHITE,
                    );
                }

                d.draw_circle(
                    rigid_body.position.x as i32,
                    rigid_body.position.y as i32,
                    1.0,
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

fn vector2_to_raylib(vector2: &Vector2) -> RaylibVector2 {
    RaylibVector2 {
        x: vector2.x as f32,
        y: vector2.y as f32,
    }
}
