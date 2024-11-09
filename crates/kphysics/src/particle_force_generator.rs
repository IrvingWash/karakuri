use kmath::Vector2;

use crate::Particle;

pub fn weight(particle: &Particle, k: f64) -> Vector2 {
    Vector2::new(0.0, 9.8 * k * particle.mass)
}

pub fn drag(particle: &Particle, k: f64) -> Vector2 {
    let velocity_squared_magnitude = particle.velocity.squared_magnitude();

    if velocity_squared_magnitude <= 0.0 {
        return Vector2::ZERO;
    }

    let drag_direction = particle.velocity.to_normalized().to_scaled(-1.0);

    let drag_magnitude = k * velocity_squared_magnitude;

    drag_direction.to_scaled(drag_magnitude)
}
