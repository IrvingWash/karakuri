use std::cmp::Ordering;

use crate::RigidBody;
use kmath::Vector2;

use super::{errors::panic_checked_circle_unwrap, SeparationInfo};

#[derive(Debug)]
pub struct Contact<'a> {
    a: &'a mut RigidBody,
    b: &'a mut RigidBody,

    normal: Vector2,
    depth: f64,

    start: Vector2,
    end: Vector2,
}

impl<'a> Contact<'a> {
    #[inline]
    pub fn for_circles(a: &'a mut RigidBody, b: &'a mut RigidBody, disposition: &Vector2) -> Self {
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
            depth: end.to_subtracted(&start).magnitude(),
            start,
            end,
        }
    }

    #[inline]
    pub fn for_polygons(
        a: &'a mut RigidBody,
        b: &'a mut RigidBody,
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
                    depth,
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
                    depth,
                    normal,
                }
            }
        }
    }

    #[inline]
    pub fn for_circle_and_polygon(
        circular: &'a mut RigidBody,
        polygonal: &'a mut RigidBody,
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
            depth,
            start,
            normal,
        }
    }

    #[inline]
    pub fn resolve_collision(mut self) {
        if self.a.can_be_rotated() || self.b.can_be_rotated() {
            self.resolve_collision_with_rotation();

            return;
        }

        self.resolve_collision_without_rotation();
    }

    fn resolve_collision_without_rotation(&mut self) {
        if self.a.is_static() && self.b.is_static() {
            return;
        }

        self.resolve_penetration();

        let elasticity = self.a.bounciness().min(self.b.bounciness());

        let relative_velocity = self.a.velocity().to_subtracted(self.b.velocity());

        let impulse_magnitude = -(1.0 + elasticity) * relative_velocity.dot_product(&self.normal)
            / (self.a.inverse_mass() + self.b.inverse_mass());

        let result = self.normal.to_scaled(impulse_magnitude);

        self.a.apply_linear_impulse(&result);
        self.b.apply_linear_impulse(&result.to_scaled(-1.0));
    }

    fn resolve_collision_with_rotation(&mut self) {
        if self.a.is_static() && self.b.is_static() {
            return;
        }

        self.resolve_penetration();

        let elasticity = self.a.bounciness().min(self.b.bounciness());
        let angular_friction = self.a.angular_friction().min(self.b.angular_friction());

        let ra = self.end.to_subtracted(self.a.position());
        let rb = self.start.to_subtracted(self.b.position());
        let va = self.a.velocity().to_added(&Vector2::new(
            -self.a.angular_velocity() * ra.y,
            self.a.angular_velocity() * ra.x,
        ));
        let vb = self.b.velocity().to_added(&Vector2::new(
            -self.b.angular_velocity() * rb.y,
            self.b.angular_velocity() * rb.x,
        ));

        let relative_velocity = va.to_subtracted(&vb);

        // Impulse along the normal
        let impulse_magnitude_along_normal = -(1.0 + elasticity)
            * relative_velocity.dot_product(&self.normal)
            / ((self.a.inverse_mass() + self.b.inverse_mass())
                + (ra.cross_product(&self.normal).powi(2) * self.a.inverse_moment_of_inertia())
                + (rb.cross_product(&self.normal).powi(2) * self.b.inverse_moment_of_inertia()));
        let impulse_along_normal = self.normal.to_scaled(impulse_magnitude_along_normal);

        // Impulse along the tangent
        let tangent = self.normal.create_perpendicular();
        let impulse_magnitude_along_tangent =
            angular_friction * -(1.0 + elasticity) * relative_velocity.dot_product(&tangent)
                / ((self.a.inverse_mass() + self.b.inverse_mass())
                    + (ra.cross_product(&tangent).powi(2) * self.a.inverse_moment_of_inertia())
                    + (rb.cross_product(&tangent).powi(2) * self.b.inverse_moment_of_inertia()));
        let impulse_along_tangent = tangent.to_scaled(impulse_magnitude_along_tangent);

        let result = impulse_along_normal.to_added(&impulse_along_tangent);

        self.a.apply_impulse_at_point(&result, &ra);
        self.b.apply_impulse_at_point(&result.to_scaled(-1.0), &rb);
    }

    fn resolve_penetration(&mut self) {
        let factor = self.depth / (self.a.inverse_mass() + self.b.inverse_mass());

        let da = factor * self.a.inverse_mass();
        let db = factor * self.b.inverse_mass();

        self.a.position_mut().subtract(&self.normal.to_scaled(da));
        self.b.position_mut().add(&self.normal.to_scaled(db));

        self.a.update_shape_vertices();
        self.b.update_shape_vertices();
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

        let contact = are_colliding(&mut a, &mut b).unwrap();

        assert_eq!(contact.end, Vector2 { x: 10.0, y: 10.0 });
        assert_eq!(contact.normal, Vector2::ZERO);
        assert_eq!(contact.start, Vector2 { x: 10.0, y: 10.0 });
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

        let contact = are_colliding(&mut a, &mut b).unwrap();

        assert_eq!(contact.end, Vector2 { x: 11.5, y: 11.5 });
        assert_eq!(contact.normal, Vector2 { x: -0.0, y: 1.0 });
        assert_eq!(contact.start, Vector2 { x: 11.5, y: 5.0 });
    }
}
