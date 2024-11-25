use kmath::Vector2;
use kphysics::{
    constraints::Constraint, shapes::Shape, RigidBody, RigidBodyParams, Simulator, SimulatorParams,
};
use raylib::{
    color::Color, consts::MouseButton, math::Vector2 as RaylibVector2, prelude::RaylibDraw,
    RaylibHandle, RaylibThread,
};

const PIXELS_PER_METER: f64 = 50.0;

#[derive(Debug)]
pub struct App {
    rl: RaylibHandle,
    thread: RaylibThread,
    running: bool,
    rigid_bodies: Vec<RigidBody>,
    simulator: Simulator,
}

impl App {
    pub fn new() -> Self {
        let (handle, thread) = raylib::init().title("Canary").size(1340, 800).build();

        Self {
            running: false,
            rl: handle,
            thread,
            rigid_bodies: Vec::new(),
            simulator: Simulator::new(SimulatorParams {
                gravity_k: PIXELS_PER_METER,
            }),
        }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn setup(&mut self) {
        self.rl.set_target_fps(60);

        let width = self.rl.get_screen_width() as f64;
        let height = self.rl.get_screen_height() as f64;

        let big_ball = RigidBody::new(RigidBodyParams {
            shape: Shape::new_circle(64.0),
            position: Vector2::new(width / 2.0, height / 2.0),
            mass: 0.0,
            can_be_rotated: true,
            bounciness: 0.9,
            ..Default::default()
        });

        let floor = RigidBody::new(RigidBodyParams {
            shape: Shape::new_rectangle(width - 50.0, 50.0),
            position: Vector2::new(width / 2.0, height - 50.0),
            mass: 0.0,
            bounciness: 0.9,
            ..Default::default()
        });

        let left_wall = RigidBody::new(RigidBodyParams {
            shape: Shape::new_rectangle(50.0, height - 100.0),
            position: Vector2::new(50.0, height / 2.0 - 25.0),
            mass: 0.0,
            bounciness: 0.9,
            ..Default::default()
        });

        let right_wall = RigidBody::new(RigidBodyParams {
            shape: Shape::new_rectangle(50.0, height - 100.0),
            position: Vector2::new(width - 50.0, height / 2.0 - 25.0),
            mass: 0.0,
            bounciness: 0.9,
            ..Default::default()
        });

        self.rigid_bodies.push(big_ball);
        self.rigid_bodies.push(floor);
        self.rigid_bodies.push(left_wall);
        self.rigid_bodies.push(right_wall);

        self.running = true;
    }

    pub fn input(&mut self) {
        self.running = !self.rl.window_should_close();

        if self
            .rl
            .is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
        {
            let mouse_position = self.rl.get_mouse_position();

            self.rigid_bodies.push(RigidBody::new(RigidBodyParams {
                shape: Shape::new_polygon(vec![
                    Vector2::new(20.0, 60.0),
                    Vector2::new(-40.0, 20.0),
                    Vector2::new(-20.0, -60.0),
                    Vector2::new(20.0, -60.0),
                    Vector2::new(40.0, 20.0),
                ]),
                position: Vector2::new(mouse_position.x.into(), mouse_position.y.into()),
                bounciness: 0.9,
                angular_friction: 0.7,
                mass: 2.0,
                can_be_rotated: true,
                ..Default::default()
            }));
        }

        if self
            .rl
            .is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_RIGHT)
        {
            let mouse_position = self.rl.get_mouse_position();

            self.rigid_bodies.push(RigidBody::new(RigidBodyParams {
                shape: Shape::new_circle(50.0),
                position: Vector2::new(mouse_position.x.into(), mouse_position.y.into()),
                bounciness: 0.9,
                angular_friction: 0.7,
                mass: 2.0,
                can_be_rotated: true,
                ..Default::default()
            }));
        }
    }

    pub fn update(&mut self) {
        let delta_time = self.rl.get_frame_time();

        self.simulator
            .update(&mut self.rigid_bodies, delta_time.into());
    }

    pub fn render(&mut self) {
        let mut d = self.rl.begin_drawing(&self.thread);

        d.clear_background(Color::BLACK);

        d.draw_fps(700, 500);

        for constraint in self.simulator.constraints() {
            match constraint {
                Constraint::Joint(joint) => {
                    let a = self
                        .rigid_bodies
                        .iter()
                        .find(|rb| rb.id() == joint.a_id)
                        .unwrap();

                    let b = self
                        .rigid_bodies
                        .iter()
                        .find(|rb| rb.id() == joint.b_id)
                        .unwrap();

                    d.draw_line(
                        a.position().x as i32,
                        a.position().y as i32,
                        b.position().x as i32,
                        b.position().y as i32,
                        Color::WHITE,
                    );
                }
                _ => {}
            }
        }

        for body in &self.rigid_bodies {
            if body.shape().is_circle() {
                let radius = body.shape().circle().unwrap().radius();

                d.draw_line(
                    body.position().x as i32,
                    body.position().y as i32,
                    (body.position().x + body.rotation().cos() * radius) as i32,
                    (body.position().y + body.rotation().sin() * radius) as i32,
                    Color::WHITE,
                );
                d.draw_circle_lines(
                    body.position().x as i32,
                    body.position().y as i32,
                    radius as f32,
                    Color::WHITE,
                );
            }

            if let Some(polygon) = body.shape().polygon() {
                for i in 0..polygon.world_vertices().len() {
                    let current = i;
                    let next = (i + 1) % polygon.world_vertices().len();

                    d.draw_line_ex(
                        vector2_to_raylib(&polygon.world_vertices()[current]),
                        vector2_to_raylib(&polygon.world_vertices()[next]),
                        1.0,
                        Color::WHITE,
                    );
                }

                d.draw_circle(
                    body.position().x as i32,
                    body.position().y as i32,
                    1.0,
                    Color::WHITE,
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
