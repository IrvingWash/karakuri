use kmath::Vector2;

use crate::Particle;

pub fn weight(particle: &Particle, k: f64) -> Vector2 {
    Vector2::new(0.0, 9.8 * k * particle.mass)
}

pub fn drag(particle: &Particle, k: f64) -> Vector2 {
    // TODO: Epsilon
    if particle.velocity == Vector2::ZERO {
        return Vector2::ZERO;
    }

    let velocity_squared_magnitude = particle.velocity.squared_magnitude();

    if velocity_squared_magnitude <= 0.0 {
        return Vector2::ZERO;
    }

    let drag_direction = particle.velocity.to_normalized().to_scaled(-1.0);

    let drag_magnitude = k * velocity_squared_magnitude;

    drag_direction.to_scaled(drag_magnitude)
}

pub fn friction(particle: &Particle, k: f64) -> Vector2 {
    // TODO: epsilon
    if particle.velocity == Vector2::ZERO {
        return Vector2::ZERO;
    }

    let friction_direction = particle.velocity.to_normalized().to_scaled(-1.0);

    let friction_magnitude = k;

    friction_direction.to_scaled(friction_magnitude)
}
