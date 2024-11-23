use kmath::Vector2;

use crate::{collisions::collision_detector, force_generator, RigidBody};

#[derive(Debug)]
pub struct WorldParams {
    pub gravity_k: f64,
}

impl Default for WorldParams {
    #[inline]
    fn default() -> Self {
        Self { gravity_k: 9.8 }
    }
}

#[derive(Debug)]
pub struct World {
    gravity_k: f64,
    rigid_bodies: Vec<RigidBody>,
    // TODO: Just store sums
    forces: Vec<Vector2>,
    torques: Vec<f64>,
}

// TODO: This API doesn't suite us
impl World {
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(params: WorldParams) -> Self {
        let WorldParams { gravity_k } = params;

        Self {
            rigid_bodies: Vec::new(),
            gravity_k,
            forces: Vec::new(),
            torques: Vec::new(),
        }
    }

    #[inline]
    pub fn rigid_bodies(&self) -> &Vec<RigidBody> {
        &self.rigid_bodies
    }

    #[inline]
    pub fn add_rigid_body(&mut self, rigid_body: RigidBody) {
        self.rigid_bodies.push(rigid_body);
    }

    #[inline]
    pub fn add_force(&mut self, force: Vector2) {
        self.forces.push(force);
    }

    #[inline]
    pub fn add_torque(&mut self, torque: f64) {
        self.torques.push(torque);
    }

    #[inline]
    pub fn update(&mut self, delta_time: f64) {
        for body in &mut self.rigid_bodies {
            let weight_force = force_generator::weight(body, self.gravity_k);
            body.apply_force(&weight_force);

            for force in &self.forces {
                body.apply_force(force);
            }

            for torque in &self.torques {
                body.apply_torque(*torque);
            }

            body.update(delta_time);
        }

        self.check_collisions();
    }

    fn check_collisions(&mut self) {
        for i in 0..self.rigid_bodies.len() {
            for j in i + 1..self.rigid_bodies.len() {
                let (first_half, second_half) = self.rigid_bodies.split_at_mut(i + 1);

                let body = first_half
                    .last_mut()
                    .expect("Should have been able to split vector in two pieces.");
                let other = &mut second_half[j - i - 1];

                if let Some(contact) = collision_detector::are_colliding(body, other) {
                    contact.resolve_collision();
                }
            }
        }
    }
}
