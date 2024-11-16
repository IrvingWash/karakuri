use kmath::Vector2;
use kphysics::{
    shapes::{Circle, Polygon, Shape},
    RigidBody, RigidBodyParams, World, WorldParams,
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
    world: World,
}

impl App {
    pub fn new() -> Self {
        let (handle, thread) = raylib::init().title("Canary").size(1340, 800).build();

        Self {
            running: false,
            rl: handle,
            thread,
            world: World::new(WorldParams {
                gravity_k: PIXELS_PER_METER,
            }),
        }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn setup(&mut self) {
        self.rl.set_target_fps(60);

        let height = self.rl.get_screen_height() as f64;
        let width = self.rl.get_screen_width() as f64;

        // Floor
        self.world.add_rigid_body(RigidBody::new(RigidBodyParams {
            shape: Shape::Polygon(Polygon::rectangular(width - 50.0, 50.0)),
            position: Vector2::new(width / 2.0, height - 50.0),
            mass: 0.0,
            bounciness: 0.5,
            ..Default::default()
        }));
        // Left wall
        self.world.add_rigid_body(RigidBody::new(RigidBodyParams {
            shape: Shape::Polygon(Polygon::rectangular(50.0, height - 100.0)),
            position: Vector2::new(50.0, height / 2.0 - 25.0),
            mass: 0.0,
            bounciness: 0.2,
            ..Default::default()
        }));
        // Right wall
        self.world.add_rigid_body(RigidBody::new(RigidBodyParams {
            shape: Shape::Polygon(Polygon::rectangular(50.0, height - 100.0)),
            position: Vector2::new(width - 50.0, height / 2.0 - 25.0),
            mass: 0.0,
            bounciness: 0.2,
            ..Default::default()
        }));
        // Big box
        self.world.add_rigid_body(RigidBody::new(RigidBodyParams {
            shape: Shape::Polygon(Polygon::rectangular(200.0, 200.0)),
            position: Vector2::new(width / 2.0, height / 2.0),
            mass: 0.0,
            bounciness: 0.7,
            rotation: 1.4,
            ..Default::default()
        }));

        // Wind force
        self.world
            .add_force(Vector2::new(0.5 * PIXELS_PER_METER, 0.0));

        self.running = true;
    }

    pub fn input(&mut self) {
        self.running = !self.rl.window_should_close();

        if self
            .rl
            .is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
        {
            let mouse_position = self.rl.get_mouse_position();

            self.world.add_rigid_body(RigidBody::new(RigidBodyParams {
                shape: Shape::Polygon(Polygon::new(vec![
                    Vector2::new(20.0, 60.0),
                    Vector2::new(-40.0, 20.0),
                    Vector2::new(-20.0, -60.0),
                    Vector2::new(20.0, -60.0),
                    Vector2::new(40.0, 20.0),
                ])),
                position: Vector2::new(mouse_position.x.into(), mouse_position.y.into()),
                bounciness: 0.1,
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

            self.world.add_rigid_body(RigidBody::new(RigidBodyParams {
                shape: Shape::Circle(Circle::new(50.0)),
                position: Vector2::new(mouse_position.x.into(), mouse_position.y.into()),
                bounciness: 0.1,
                angular_friction: 0.7,
                mass: 2.0,
                can_be_rotated: true,
                ..Default::default()
            }));
        }
    }

    pub fn update(&mut self) {
        let delta_time = self.rl.get_frame_time();

        self.world.update(delta_time.into());
    }

    pub fn render(&mut self) {
        let mut d = self.rl.begin_drawing(&self.thread);

        d.clear_background(Color::BLACK);

        for body in self.world.rigid_bodies() {
            if body.shape.is_circle() {
                let radius = body.shape.circle().unwrap().radius;

                d.draw_line(
                    body.position.x as i32,
                    body.position.y as i32,
                    (body.position.x + body.rotation.cos() * radius) as i32,
                    (body.position.y + body.rotation.sin() * radius) as i32,
                    Color::WHITE,
                );
                d.draw_circle_lines(
                    body.position.x as i32,
                    body.position.y as i32,
                    radius as f32,
                    Color::WHITE,
                );
            }

            if let Some(polygon) = body.shape.polygon() {
                for i in 0..polygon.world_vertices.len() {
                    let current = i;
                    let next = (i + 1) % polygon.world_vertices.len();

                    d.draw_line_ex(
                        vector2_to_raylib(&polygon.world_vertices[current]),
                        vector2_to_raylib(&polygon.world_vertices[next]),
                        1.0,
                        Color::WHITE,
                    );
                }

                d.draw_circle(
                    body.position.x as i32,
                    body.position.y as i32,
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
