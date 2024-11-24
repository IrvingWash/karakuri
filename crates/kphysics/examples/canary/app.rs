use kmath::Vector2;
use kphysics::{
    constraints::Constraint, shapes::Shape, RigidBody, RigidBodyParams, Simulator, SimulatorParams,
};
use raylib::{
    color::Color, math::Vector2 as RaylibVector2, prelude::RaylibDraw, RaylibHandle, RaylibThread,
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
        let (handle, thread) = raylib::init().title("Caeary").size(1340, 800).build();

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
        let _height = self.rl.get_screen_height() as f64;

        const BODY_COUNT: usize = 8;

        for i in 0..BODY_COUNT {
            let mass = if i == 0 { 0.0 } else { 1.0 };

            let body = RigidBody::new(RigidBodyParams {
                shape: Shape::new_rectangle(30.0, 30.0),
                position: Vector2::new(width / 2.0 - i as f64 * 40.0, 100.0),
                mass,
                can_be_rotated: true,
                ..Default::default()
            });

            self.rigid_bodies.push(body);
        }

        for i in 0..BODY_COUNT - 1 {
            let a = &self.rigid_bodies[i];
            let b = &self.rigid_bodies[i + 1];

            let joint = Constraint::new_joint(a, b, a.position());

            self.simulator.add_constraint(joint);
        }

        self.running = true;
    }

    pub fn input(&mut self) {
        self.running = !self.rl.window_should_close();
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
