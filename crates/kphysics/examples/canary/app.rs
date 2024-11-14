use kmath::Vector2;
use kphysics::{
    collisions::collision_detector,
    force_generator,
    shapes::{Polygon, Shape},
    RigidBody,
};
use raylib::{
    color::Color, consts::MouseButton, math::Vector2 as RaylibVector2, prelude::RaylibDraw,
    RaylibHandle, RaylibThread,
};

#[allow(dead_code)]
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

        let width = self.rl.get_screen_width();
        let height = self.rl.get_screen_height();

        let floor = RigidBody::new(
            Vector2::new(width as f64 / 2.0, height as f64 - 50.0),
            0.0,
            Shape::Polygon(Polygon::rectangular(width as f64 - 50.0, 50.0)),
            Some(0.2),
        );
        let left_wall = RigidBody::new(
            Vector2::new(width as f64 - 50.0, height as f64 / 2.0 - 25.0),
            0.0,
            Shape::Polygon(Polygon::rectangular(50.0, height as f64 - 100.0)),
            Some(0.2),
        );
        let right_wall = RigidBody::new(
            Vector2::new(50.0, height as f64 / 2.0 - 25.0),
            0.0,
            Shape::Polygon(Polygon::rectangular(50.0, height as f64 - 100.0)),
            Some(0.2),
        );
        let mut big_box = RigidBody::new(
            Vector2::new(width as f64 / 2.0, height as f64 / 2.0),
            0.0,
            Shape::Polygon(Polygon::rectangular(200.0, 200.0)),
            Some(0.5),
        );

        big_box.rotation = 1.4;

        self.rigid_bodies.push(floor);
        self.rigid_bodies.push(left_wall);
        self.rigid_bodies.push(right_wall);
        self.rigid_bodies.push(big_box);

        self.running = true;
    }

    pub fn input(&mut self) {
        self.running = !self.rl.window_should_close();

        if self
            .rl
            .is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
        {
            let mouse_position = self.rl.get_mouse_position();

            let mut cube = RigidBody::new(
                Vector2::new(mouse_position.x.into(), mouse_position.y.into()),
                1.0,
                Shape::Polygon(Polygon::rectangular(50.0, 50.0)),
                None,
            );

            cube.can_be_rotated = true;

            self.rigid_bodies.push(cube);
        }
    }

    pub fn update(&mut self) {
        let delta_time = self.rl.get_frame_time();

        for rigid_body in &mut self.rigid_bodies {
            rigid_body.apply_force(&self.push_force);
            rigid_body.apply_force(&force_generator::weight(&rigid_body, PIXELS_PER_METER));

            rigid_body.is_colliding = false;

            rigid_body.update(delta_time.into());
        }

        for i in 0..self.rigid_bodies.len() {
            for j in i + 1..self.rigid_bodies.len() {
                let (f, s) = self.rigid_bodies.split_at_mut(i + 1);

                let body = f.last_mut().unwrap();
                let other = &mut s[j - i - 1];

                #[allow(unused_mut)]
                if let Some(mut contact) = collision_detector::are_colliding(body, other) {
                    contact.resolve_collision();

                    body.is_colliding = true;
                    other.is_colliding = true;
                }
            }
        }

        self.push_force.reset();
    }

    pub fn render(&mut self) {
        let mut d = self.rl.begin_drawing(&self.thread);

        d.clear_background(Color::BLACK);

        for rigid_body in &self.rigid_bodies {
            // Draw circular rigid_bodies
            if rigid_body.shape.is_circle() {
                let radius = rigid_body.shape.circle().unwrap().radius;

                d.draw_line(
                    rigid_body.position.x as i32,
                    rigid_body.position.y as i32,
                    (rigid_body.position.x + rigid_body.rotation.cos() * radius) as i32,
                    (rigid_body.position.y + rigid_body.rotation.sin() * radius) as i32,
                    if rigid_body.is_colliding {
                        Color::RED
                    } else {
                        Color::WHITE
                    },
                );
                d.draw_circle_lines(
                    rigid_body.position.x as i32,
                    rigid_body.position.y as i32,
                    radius as f32,
                    if rigid_body.is_colliding {
                        Color::RED
                    } else {
                        Color::WHITE
                    },
                );
            }

            // Draw rectangular rigid bodies
            if let Some(rectangle) = rigid_body.shape.polygon() {
                for i in 0..rectangle.world_vertices.len() {
                    let curr = i;
                    let next = (i + 1) % rectangle.world_vertices.len();

                    d.draw_line_ex(
                        vector2_to_raylib(&rectangle.world_vertices[curr]),
                        vector2_to_raylib(&rectangle.world_vertices[next]),
                        1.0,
                        if rigid_body.is_colliding {
                            Color::MAGENTA
                        } else {
                            Color::WHITE
                        },
                    );
                }

                d.draw_circle(
                    rigid_body.position.x as i32,
                    rigid_body.position.y as i32,
                    1.0,
                    if rigid_body.is_colliding {
                        Color::MAGENTA
                    } else {
                        Color::WHITE
                    },
                );
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
