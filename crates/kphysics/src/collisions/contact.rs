use std::cmp::Ordering;

use crate::RigidBody;
use kmath::Vector2;

use super::{errors::panic_checked_circle_unwrap, SeparationInfo};

#[derive(Debug)]
pub struct Contact<'a> {
    // TODO: Add getters instead of pub
    pub a: &'a RigidBody,
    pub b: &'a RigidBody,

    pub normal: Vector2,

    pub start: Vector2,
    pub end: Vector2,
}

impl<'a> Contact<'a> {
    #[inline]
    pub fn for_circles(a: &'a RigidBody, b: &'a RigidBody, disposition: &Vector2) -> Self {
        let normal = disposition.to_normalized();

        let start = b.position().to_subtracted(
            &normal.to_scaled(
                b.shape()
                    .circle()
                    .unwrap_or_else(|| panic_checked_circle_unwrap())
                    .radius(),
            ),
        );

        let end = a.position().to_added(
            &normal.to_scaled(
                a.shape()
                    .circle()
                    .unwrap_or_else(|| panic_checked_circle_unwrap())
                    .radius(),
            ),
        );

        Self {
            a,
            b,
            normal,
            start,
            end,
        }
    }

    #[inline]
    pub fn for_polygons(
        a: &'a RigidBody,
        b: &'a RigidBody,
        ab_separation_info: SeparationInfo,
        ba_separation_info: SeparationInfo,
    ) -> Self {
        match ab_separation_info
            .separation
            .total_cmp(&ba_separation_info.separation)
        {
            Ordering::Greater => {
                let depth = -ab_separation_info.separation;
                let normal = ab_separation_info.separation_axis.create_perpendicular();
                let point = ab_separation_info.point;

                Self {
                    a,
                    b,
                    end: point.to_added(&normal.to_scaled(depth)),
                    start: point,
                    normal,
                }
            }
            Ordering::Equal | Ordering::Less => {
                let depth = -ba_separation_info.separation;
                let normal = ba_separation_info
                    .separation_axis
                    .create_perpendicular()
                    .to_scaled(-1.0);
                let point = ba_separation_info.point;

                Self {
                    a,
                    b,
                    start: point.to_subtracted(&normal.to_scaled(depth)),
                    end: point,
                    normal,
                }
            }
        }
    }

    #[inline]
    pub fn for_circle_and_polygon(
        circular: &'a RigidBody,
        polygonal: &'a RigidBody,
        v1: &Vector2,
        v1_magnitude: f64,
        flip: bool,
    ) -> Self {
        let normal = v1.to_normalized();

        let circle_radius = circular
            .shape()
            .circle()
            .unwrap_or_else(|| panic_checked_circle_unwrap())
            .radius();

        let start = if !flip {
            circular
                .position()
                .to_added(&normal.to_scaled(-circle_radius))
        } else {
            circular
                .position()
                .to_subtracted(&normal.to_scaled(circle_radius))
        };

        let depth = circle_radius - v1_magnitude;

        Self {
            a: polygonal,
            b: circular,
            end: start.to_added(&normal.to_scaled(depth)),
            start,
            normal,
        }
    }
}

#[cfg(test)]
mod collision_detector_tests {
    use kmath::Vector2;

    use crate::{
        collisions::collision_detector::are_colliding, shapes::Shape, RigidBody, RigidBodyParams,
    };

    #[test]
    fn test_circles() {
        let mut a = RigidBody::new(RigidBodyParams {
            position: Vector2::new(0.0, 0.0),
            shape: Shape::new_circle(1.0),
            ..Default::default()
        });
        let mut b = RigidBody::new(RigidBodyParams {
            position: Vector2::new(10.0, 10.0),
            shape: Shape::new_circle(1.0),
            ..Default::default()
        });

        assert!(are_colliding(&mut a, &mut b).is_none());

        *a.position_mut() = Vector2::new(10.0, 10.0);

        let contacts = are_colliding(&mut a, &mut b).unwrap();

        assert_eq!(contacts[0].end, Vector2 { x: 10.0, y: 10.0 });
        assert_eq!(contacts[0].normal, Vector2::ZERO);
        assert_eq!(contacts[0].start, Vector2 { x: 10.0, y: 10.0 });
    }

    #[test]
    fn test_polygons() {
        let mut a = RigidBody::new(RigidBodyParams {
            position: Vector2::new(0.0, 0.0),
            shape: Shape::new_rectangle(5.0, 5.0),
            ..Default::default()
        });
        let mut b = RigidBody::new(RigidBodyParams {
            position: Vector2::new(10.0, 10.0),
            shape: Shape::new_rectangle(10.0, 10.0),
            ..Default::default()
        });

        assert!(are_colliding(&mut a, &mut b).is_none());

        *a.position_mut() = Vector2::new(9.0, 9.0);

        a.integrate_forces(1.0);
        a.integrate_velocities(1.0);
        b.integrate_forces(1.0);
        b.integrate_velocities(1.0);

        let contacts = are_colliding(&mut a, &mut b).unwrap();

        assert_eq!(contacts[0].end, Vector2 { x: 11.5, y: 11.5 });
        assert_eq!(contacts[0].normal, Vector2 { x: -0.0, y: 1.0 });
        assert_eq!(contacts[0].start, Vector2 { x: 11.5, y: 5.0 });
    }
}
