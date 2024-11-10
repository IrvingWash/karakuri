use kmath::Vector2;
use kphysics::{force_generator, RigidBody};
use raylib::{
    color::Color,
    consts::{KeyboardKey, MouseButton},
    math::Vector2 as RaylibVector2,
    prelude::RaylibDraw,
    RaylibHandle, RaylibThread,
};

const PIXELS_PER_METER: f64 = 50.0;

const STIFFNESS: f64 = 1500.0;
const REST_LENGTH: f64 = 200.0;

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
        let (handle, thread) = raylib::init().title("Soft Body").size(1340, 800).build();

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

        self.rigid_bodies
            .push(RigidBody::new(Vector2::new(100.0, 100.0), 1.0, 6.0));
        self.rigid_bodies
            .push(RigidBody::new(Vector2::new(300.0, 100.0), 1.0, 6.0));
        self.rigid_bodies
            .push(RigidBody::new(Vector2::new(300.0, 300.0), 1.0, 6.0));
        self.rigid_bodies
            .push(RigidBody::new(Vector2::new(100.0, 300.0), 1.0, 6.0));

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

        let ab_spring_force = force_generator::spring(
            &self.rigid_bodies[0],
            &self.rigid_bodies[1],
            200.,
            STIFFNESS,
        );
        self.rigid_bodies[0].apply_force(&ab_spring_force);
        self.rigid_bodies[1].apply_force(&ab_spring_force.to_scaled(-1.));

        let bc_spring_force = force_generator::spring(
            &self.rigid_bodies[1],
            &self.rigid_bodies[2],
            REST_LENGTH,
            STIFFNESS,
        );
        self.rigid_bodies[1].apply_force(&bc_spring_force);
        self.rigid_bodies[2].apply_force(&bc_spring_force.to_scaled(-1.));

        let cd_spring_force = force_generator::spring(
            &self.rigid_bodies[2],
            &self.rigid_bodies[3],
            REST_LENGTH,
            STIFFNESS,
        );
        self.rigid_bodies[2].apply_force(&cd_spring_force);
        self.rigid_bodies[3].apply_force(&cd_spring_force.to_scaled(-1.));

        let da_spring_force = force_generator::spring(
            &self.rigid_bodies[3],
            &self.rigid_bodies[0],
            REST_LENGTH,
            STIFFNESS,
        );
        self.rigid_bodies[3].apply_force(&da_spring_force);
        self.rigid_bodies[0].apply_force(&da_spring_force.to_scaled(-1.));

        let ac_spring_force = force_generator::spring(
            &self.rigid_bodies[0],
            &self.rigid_bodies[2],
            REST_LENGTH,
            STIFFNESS,
        );
        self.rigid_bodies[0].apply_force(&ac_spring_force);
        self.rigid_bodies[2].apply_force(&ac_spring_force.to_scaled(-1.));

        let bd_spring_force = force_generator::spring(
            &self.rigid_bodies[1],
            &self.rigid_bodies[3],
            REST_LENGTH,
            STIFFNESS,
        );
        self.rigid_bodies[1].apply_force(&bd_spring_force);
        self.rigid_bodies[3].apply_force(&bd_spring_force.to_scaled(-1.));

        for rigid_body in &mut self.rigid_bodies {
            rigid_body.apply_force(&force_generator::drag(rigid_body, 0.002));
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

        d.draw_line_ex(
            vector2_to_raylib(&self.rigid_bodies[0].position),
            vector2_to_raylib(&self.rigid_bodies[1].position),
            15.0,
            Color::GREEN,
        );
        d.draw_line_ex(
            vector2_to_raylib(&self.rigid_bodies[1].position),
            vector2_to_raylib(&self.rigid_bodies[2].position),
            15.0,
            Color::GREEN,
        );
        d.draw_line_ex(
            vector2_to_raylib(&self.rigid_bodies[2].position),
            vector2_to_raylib(&self.rigid_bodies[3].position),
            15.0,
            Color::GREEN,
        );
        d.draw_line_ex(
            vector2_to_raylib(&self.rigid_bodies[3].position),
            vector2_to_raylib(&self.rigid_bodies[0].position),
            15.0,
            Color::GREEN,
        );
        d.draw_line_ex(
            vector2_to_raylib(&self.rigid_bodies[0].position),
            vector2_to_raylib(&self.rigid_bodies[2].position),
            15.0,
            Color::GREEN,
        );
        d.draw_line_ex(
            vector2_to_raylib(&self.rigid_bodies[1].position),
            vector2_to_raylib(&self.rigid_bodies[3].position),
            15.0,
            Color::GREEN,
        );

        // Draw the rigid_bodies
        for rigid_body in &self.rigid_bodies {
            d.draw_circle_v(
                vector2_to_raylib(&rigid_body.position),
                rigid_body.radius as f32,
                Color::WHEAT,
            );
        }
    }

    fn keep_in_window(&mut self) {
        let width: f64 = self.rl.get_screen_width().into();
        let height: f64 = self.rl.get_screen_height().into();

        for rigid_body in &mut self.rigid_bodies {
            if rigid_body.position.x + rigid_body.radius >= width {
                rigid_body.position.x = width - rigid_body.radius;
                rigid_body.velocity.x *= -0.9;

                return;
            }

            if rigid_body.position.x - rigid_body.radius <= 0.0 {
                rigid_body.position.x = rigid_body.radius;
                rigid_body.velocity.x *= -0.9;

                return;
            }

            if rigid_body.position.y + rigid_body.radius >= height {
                rigid_body.position.y = height - rigid_body.radius;
                rigid_body.velocity.y *= -0.9;
            }

            if rigid_body.position.y - rigid_body.radius <= 0.0 {
                rigid_body.position.y = rigid_body.radius;
                rigid_body.velocity.y *= -0.9;
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
