use kmath::Vector2;

use crate::RigidBody;

#[inline]
pub const fn weight(rigid_body: &RigidBody, k: f64) -> Vector2 {
    if rigid_body.mass == 0.0 {
        return Vector2::ZERO;
    }

    Vector2::new(0.0, 9.8 * k * rigid_body.mass)
}

#[inline]
pub fn drag(rigid_body: &RigidBody, k: f64) -> Vector2 {
    let velocity_squared_magnitude = rigid_body.velocity.squared_magnitude();

    if velocity_squared_magnitude <= 0.0 {
        return Vector2::ZERO;
    }

    let drag_direction = rigid_body.velocity.to_normalized().to_scaled(-1.0);

    let drag_magnitude = k * velocity_squared_magnitude;

    drag_direction.to_scaled(drag_magnitude)
}

#[inline]
pub fn friction(rigid_body: &RigidBody, k: f64) -> Vector2 {
    let friction_direction = rigid_body.velocity.to_normalized().to_scaled(-1.0);

    let friction_magnitude = k;

    friction_direction.to_scaled(friction_magnitude)
}

#[inline]
pub fn gravitation(
    a: &RigidBody,
    b: &RigidBody,
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
pub fn spring(rigid_body: &RigidBody, anchor: &RigidBody, rest_length: f64, k: f64) -> Vector2 {
    let disposition = rigid_body.position.to_subtracted(&anchor.position);

    let displacement = disposition.magnitude() - rest_length;

    let spring_direction = disposition.to_normalized();

    let spring_magnitude = -k * displacement;

    spring_direction.to_scaled(spring_magnitude)
}
