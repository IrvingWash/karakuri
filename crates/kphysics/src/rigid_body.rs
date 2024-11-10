use kmath::Vector2;

#[derive(Debug)]
pub struct RigidBody {
    pub position: Vector2,
    pub velocity: Vector2,
    pub mass: f64,
    pub inverse_mass: f64,
    pub radius: f64, // For debug
    pub accumulated_forces: Vector2,
}

impl Default for RigidBody {
    fn default() -> Self {
        Self {
            position: Vector2::ZERO,
            velocity: Vector2::ZERO,
            mass: 1.0,
            inverse_mass: 1.0,
            radius: 0.0,
            accumulated_forces: Vector2::ZERO,
        }
    }
}

impl RigidBody {
    #[inline]
    pub const fn new(position: Vector2, mass: f64, radius: f64) -> Self {
        Self {
            position,
            velocity: Vector2::ZERO,
            mass,
            inverse_mass: if mass != 0.0 { 1.0 / mass } else { 0.0 },
            radius,
            accumulated_forces: Vector2::ZERO,
        }
    }

    #[inline]
    pub fn apply_force(&mut self, force: &Vector2) {
        self.accumulated_forces.add(force);
    }

    #[inline]
    pub fn integrate(&mut self, delta_time: f64) {
        let acceleration = self.accumulated_forces.to_scaled(self.inverse_mass);

        self.velocity.add(&acceleration.to_scaled(delta_time));

        self.position.add(&self.velocity.to_scaled(delta_time));

        self.clear_forces();
    }

    fn clear_forces(&mut self) {
        self.accumulated_forces.reset();
    }
}
