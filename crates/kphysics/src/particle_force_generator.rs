use kmath::Vector2;

use crate::Particle;

#[inline]
pub const fn weight(particle: &Particle, k: f64) -> Vector2 {
    if particle.mass == 0.0 {
        return Vector2::ZERO;
    }

    Vector2::new(0.0, 9.8 * k * particle.mass)
}

#[inline]
pub fn drag(particle: &Particle, k: f64) -> Vector2 {
    let velocity_squared_magnitude = particle.velocity.squared_magnitude();

    if velocity_squared_magnitude <= 0.0 {
        return Vector2::ZERO;
    }

    let drag_direction = particle.velocity.to_normalized().to_scaled(-1.0);

    let drag_magnitude = k * velocity_squared_magnitude;

    drag_direction.to_scaled(drag_magnitude)
}

#[inline]
pub fn friction(particle: &Particle, k: f64) -> Vector2 {
    let friction_direction = particle.velocity.to_normalized().to_scaled(-1.0);

    let friction_magnitude = k;

    friction_direction.to_scaled(friction_magnitude)
}

#[inline]
pub fn gravitation(
    a: &Particle,
    b: &Particle,
    g: f64,
    min_distance: f64,
    max_distance: f64,
) -> Vector2 {
    if a.mass == 0.0 && b.mass == 0.0 {
        return Vector2::ZERO;
    }

    let disposition = b.position.to_subtracted(&a.position);

    let squared_distance = disposition
        .squared_magnitude()
        .clamp(min_distance, max_distance);

    let attraction_direction = disposition.to_normalized();

    let attraction_magnitude = g * (a.mass * b.mass) / squared_distance;

    attraction_direction.to_scaled(attraction_magnitude)
}

#[inline]
pub fn spring(particle: &Particle, anchor: &Particle, rest_length: f64, k: f64) -> Vector2 {
    let disposition = particle.position.to_subtracted(&anchor.position);

    let displacement = disposition.magnitude() - rest_length;

    let spring_direction = disposition.to_normalized();

    let spring_magnitude = -k * displacement;

    spring_direction.to_scaled(spring_magnitude)
}
