use kmath::Vector2;

use crate::RigidBody;

#[inline]
pub fn weight(rigid_body: &RigidBody, k: f64) -> Vector2 {
    if rigid_body.mass() == 0.0 {
        return Vector2::ZERO;
    }

    Vector2::new(0.0, 9.8 * k * rigid_body.mass())
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
    if a.mass() == 0.0 && b.mass() == 0.0 {
        return Vector2::ZERO;
    }

    let disposition = b.position.to_subtracted(&a.position);

    let squared_distance = disposition
        .squared_magnitude()
        .clamp(min_distance, max_distance);

    let attraction_direction = disposition.to_normalized();

    let attraction_magnitude = g * (a.mass() * b.mass()) / squared_distance;

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

#[cfg(test)]
mod force_generator_tests {
    use kmath::Vector2;

    use crate::{
        force_generator::{drag, friction, gravitation, spring, weight},
        rigid_body::RigidBodyParams,
        RigidBody,
    };

    #[test]
    fn test_weight() {
        let rb = RigidBody::new(RigidBodyParams {
            mass: 0.0,
            ..Default::default()
        });

        assert_eq!(weight(&rb, 50.0), Vector2::ZERO);

        let rb = RigidBody::new(RigidBodyParams {
            mass: 3.0,
            ..Default::default()
        });

        assert_eq!(weight(&rb, 50.0), Vector2::new(0.0, 1470.0000000000002));
    }

    #[test]
    fn test_drag() {
        let rb = RigidBody::new(RigidBodyParams {
            mass: 0.0,
            velocity: Vector2::new(3.0, 3.0),
            ..Default::default()
        });

        assert_eq!(
            drag(&rb, 50.0),
            Vector2 {
                x: -636.3961030678928,
                y: -636.3961030678928
            }
        );

        let rb = RigidBody::new(RigidBodyParams {
            mass: 3.0,
            velocity: Vector2::new(3.0, 3.0),
            ..Default::default()
        });

        assert_eq!(
            drag(&rb, 50.0),
            Vector2 {
                x: -636.3961030678928,
                y: -636.3961030678928
            }
        );
    }

    #[test]
    fn test_friction() {
        let rb = RigidBody::new(RigidBodyParams {
            mass: 0.0,
            velocity: Vector2::new(3.0, 3.0),
            ..Default::default()
        });

        assert_eq!(
            friction(&rb, 50.0),
            Vector2 {
                x: -35.35533905932738,
                y: -35.35533905932738
            }
        );

        let rb = RigidBody::new(RigidBodyParams {
            mass: 3.0,
            velocity: Vector2::new(3.0, 3.0),
            ..Default::default()
        });

        assert_eq!(
            friction(&rb, 50.0),
            Vector2 {
                x: -35.35533905932738,
                y: -35.35533905932738
            }
        );
    }

    #[test]
    fn test_gravitation() {
        let rba = RigidBody::new(RigidBodyParams {
            mass: 0.0,
            ..Default::default()
        });

        let rbb = RigidBody::new(RigidBodyParams {
            position: Vector2::new(5.0, 5.0),
            mass: 0.0,
            ..Default::default()
        });

        assert_eq!(gravitation(&rba, &rbb, 50.0, 10.0, 500.0), Vector2::ZERO);

        let rba = RigidBody::new(RigidBodyParams {
            mass: 3.0,
            ..Default::default()
        });

        let rbb = RigidBody::new(RigidBodyParams {
            position: Vector2::new(5.0, 5.0),
            mass: 0.5,
            ..Default::default()
        });

        let gravitation_force = gravitation(&rba, &rbb, 50.0, 10.0, 500.0);

        assert_eq!(
            gravitation_force,
            Vector2 {
                x: 1.0606601717798212,
                y: 1.0606601717798212
            }
        );
    }

    #[test]
    fn test_spring() {
        let rba = RigidBody::new(RigidBodyParams {
            mass: 0.0,
            ..Default::default()
        });

        let rbb = RigidBody::new(RigidBodyParams {
            position: Vector2::new(5.0, 5.0),
            mass: 0.0,
            ..Default::default()
        });

        assert_eq!(
            spring(&rba, &rbb, 300.0, 50.0),
            Vector2 {
                x: -10356.601717798212,
                y: -10356.601717798212
            }
        );
    }
}
