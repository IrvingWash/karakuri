use kmath::Vector2;

use crate::{collisions::collision_detector, constraints::Constraint, force_generator, RigidBody};

#[derive(Debug)]
pub struct SimulatorParams {
    pub gravity_k: f64,
}

impl Default for SimulatorParams {
    #[inline]
    fn default() -> Self {
        Self { gravity_k: 9.8 }
    }
}

#[derive(Debug)]
pub struct Simulator {
    gravity_k: f64,
    constraints: Vec<Constraint>,
    // TODO: Just store sums
    forces: Vec<Vector2>,
    torques: Vec<f64>,
}

impl Simulator {
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(params: SimulatorParams) -> Self {
        let SimulatorParams { gravity_k } = params;

        Self {
            gravity_k,
            constraints: Vec::new(),
            forces: Vec::new(),
            torques: Vec::new(),
        }
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
    pub fn add_constraint(&mut self, constraint: Constraint) {
        self.constraints.push(constraint);
    }

    #[inline]
    pub fn constraints(&self) -> &Vec<Constraint> {
        &self.constraints
    }

    #[inline]
    pub fn update(&mut self, rigid_bodies: &mut [RigidBody], delta_time: f64) {
        for body in &mut *rigid_bodies {
            let weight_force = force_generator::weight(body, self.gravity_k);
            body.apply_force(&weight_force);

            for force in &self.forces {
                body.apply_force(force);
            }

            for torque in &self.torques {
                body.apply_torque(*torque);
            }
        }

        for body in &mut *rigid_bodies {
            body.integrate_forces(delta_time);
        }

        for constraint in &mut self.constraints {
            constraint.pre_solve(rigid_bodies);
        }

        for _ in 0..5 {
            for constraint in &mut self.constraints {
                constraint.solve(rigid_bodies);
            }
        }

        for constraints in &self.constraints {
            constraints.post_solve();
        }

        for body in &mut *rigid_bodies {
            body.integrate_velocities(delta_time);
        }

        self.check_collisions(rigid_bodies);
    }

    fn check_collisions(&self, rigid_bodies: &mut [RigidBody]) {
        for i in 0..rigid_bodies.len() {
            for j in i + 1..rigid_bodies.len() {
                let (first_half, second_half) = rigid_bodies.split_at_mut(i + 1);

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
