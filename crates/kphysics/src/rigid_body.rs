use kmath::Vector2;

use crate::shapes::Shape;

#[derive(Debug)]
pub struct RigidBody {
    pub shape: Shape,

    // Linear motion
    pub position: Vector2,
    pub velocity: Vector2,
    pub accumulated_forces: Vector2,

    // Angular motion
    pub rotation: f64,
    pub angular_velocity: f64,
    pub accumulated_torque: f64,

    pub mass: f64,
    pub inverse_mass: f64,
    pub moment_of_inertia: f64,
    pub inverse_moment_of_inertia: f64,
}

impl RigidBody {
    #[inline]
    pub fn new(position: Vector2, mass: f64, shape: Shape) -> Self {
        let moment_of_inertia = shape.moment_of_inertia() * mass;

        Self {
            shape,
            position,
            velocity: Vector2::ZERO,
            accumulated_forces: Vector2::ZERO,
            rotation: 0.0,
            angular_velocity: 0.0,
            accumulated_torque: 0.0,
            mass,
            inverse_mass: if mass != 0.0 { 1.0 / mass } else { 0.0 },
            moment_of_inertia,
            inverse_moment_of_inertia: if moment_of_inertia == 0.0 {
                0.0
            } else {
                1.0 / moment_of_inertia
            },
        }
    }

    #[inline]
    pub fn apply_force(&mut self, force: &Vector2) {
        self.accumulated_forces.add(force);
    }

    #[inline]
    pub fn apply_torque(&mut self, torque: f64) {
        self.accumulated_torque += torque;
    }

    #[inline]
    pub fn integrate_linear(&mut self, delta_time: f64) {
        let acceleration = self.accumulated_forces.to_scaled(self.inverse_mass);

        self.velocity.add(&acceleration.to_scaled(delta_time));

        self.position.add(&self.velocity.to_scaled(delta_time));

        self.clear_forces();
    }

    #[inline]
    pub fn integrate_angular(&mut self, delta_time: f64) {
        let angular_acceleration = self.accumulated_torque * self.inverse_moment_of_inertia;

        self.angular_velocity += angular_acceleration * delta_time;

        self.rotation += self.angular_velocity * delta_time;

        self.clear_torque();
    }

    // TODO: integrates and this one should be combined into single `update`
    #[inline]
    pub fn update_vertices(&mut self) {
        self.shape.update_vertices(&self.position, self.rotation);
    }

    fn clear_forces(&mut self) {
        self.accumulated_forces.reset();
    }

    fn clear_torque(&mut self) {
        self.accumulated_torque = 0.0;
    }
}
