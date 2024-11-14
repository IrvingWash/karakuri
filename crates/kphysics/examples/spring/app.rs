use kmath::Vector2;
use kphysics::{
    force_generator,
    shapes::{Circle, Shape},
    RigidBody, RigidBodyParams,
};
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

    rigid_bodies: Vec<RigidBody>,

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

        // Anchor
        self.rigid_bodies.push(RigidBody::new(RigidBodyParams {
            position: Vector2::new((self.rl.get_screen_width() / 2).into(), 10.0),
            shape: Shape::Circle(Circle::new(5.0)),
            ..Default::default()
        }));

        // Bobs
        for i in 1..=15 {
            self.rigid_bodies.push(RigidBody::new(RigidBodyParams {
                position: Vector2::new(600.0, f64::from(i) * 15.0),
                mass: 2.0,
                shape: Shape::Circle(Circle::new(5.0)),
                ..Default::default()
            }))
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
                Vector2::new(self.mouse_position.x.into(), self.mouse_position.y.into());

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

        for i in 1..self.rigid_bodies.len() {
            let current = &self.rigid_bodies[i];
            let previous = &self.rigid_bodies[i - 1];

            let spring_force = force_generator::spring(current, previous, 15.0, 300.0);

            self.rigid_bodies[i].apply_force(&spring_force);
            self.rigid_bodies[i - 1].apply_force(&spring_force.to_scaled(-1.0));
        }

        for rigid_body in &mut self.rigid_bodies {
            rigid_body.apply_force(&force_generator::drag(rigid_body, 0.002));
            rigid_body.apply_force(&force_generator::weight(rigid_body, PIXELS_PER_METER));
            rigid_body.apply_force(&self.push_force);

            rigid_body.update(delta_time.into());
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

        // Draw spring between rigid_bodies
        for i in 1..self.rigid_bodies.len() {
            d.draw_line_ex(
                vector2_to_raylib(&self.rigid_bodies[i - 1].position),
                vector2_to_raylib(&self.rigid_bodies[i].position),
                15.0,
                Color::GREEN,
            );
        }

        // Draw the rigid_bodies
        for rigid_body in &self.rigid_bodies {
            if rigid_body.shape.is_circle() {
                let radius = rigid_body.shape.circle().unwrap().radius;

                d.draw_circle_v(
                    vector2_to_raylib(&rigid_body.position),
                    radius as f32,
                    Color::WHEAT,
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
