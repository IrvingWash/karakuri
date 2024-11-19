use std::cmp::Ordering;

use crate::RigidBody;
use kmath::Vector2;

use super::{errors::panic_checked_circle_unwrap, SeparationInfo};

#[derive(Debug)]
pub struct Contact<'a> {
    a: &'a mut RigidBody,
    b: &'a mut RigidBody,

    pub normal: Vector2, // TODO: It's pub for debug
    depth: f64,

    pub start: Vector2, // TODO: It's pub for debug
    pub end: Vector2,   // TODO: It's pub for debug
}

impl<'a> Contact<'a> {
    #[inline]
    pub fn for_circles(a: &'a mut RigidBody, b: &'a mut RigidBody, disposition: &Vector2) -> Self {
        let normal = disposition.to_normalized();

        let start = b.position.to_subtracted(
            &normal.to_scaled(
                b.shape
                    .circle()
                    .unwrap_or_else(|| panic_checked_circle_unwrap())
                    .radius,
            ),
        );

        let end = a.position.to_added(
            &normal.to_scaled(
                a.shape
                    .circle()
                    .unwrap_or_else(|| panic_checked_circle_unwrap())
                    .radius,
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
            .shape
            .circle()
            .unwrap_or_else(|| panic_checked_circle_unwrap())
            .radius;

        let start = if !flip {
            circular
                .position
                .to_added(&normal.to_scaled(-circle_radius))
        } else {
            circular
                .position
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
        if self.a.can_be_rotated || self.b.can_be_rotated {
            self.resolve_collision_with_rotation();

            return;
        }

        self.resolve_collision_without_rotation();
    }

    fn resolve_collision_without_rotation(&mut self) {
        if self.a.is_static && self.b.is_static {
            return;
        }

        self.resolve_penetration();

        let elasticity = self.a.bounciness.min(self.b.bounciness);

        let relative_velocity = self.a.velocity.to_subtracted(&self.b.velocity);

        let impulse_magnitude = -(1.0 + elasticity) * relative_velocity.dot_product(&self.normal)
            / (self.a.inverse_mass + self.b.inverse_mass);

        let result = self.normal.to_scaled(impulse_magnitude);

        self.a.apply_impulse(&result);
        self.b.apply_impulse(&result.to_scaled(-1.0));
    }

    fn resolve_collision_with_rotation(&mut self) {
        if self.a.is_static && self.b.is_static {
            return;
        }

        self.resolve_penetration();

        let elasticity = self.a.bounciness.min(self.b.bounciness);
        let angular_friction = self.a.angular_friction.min(self.b.angular_friction);

        let ra = self.end.to_subtracted(&self.a.position);
        let rb = self.start.to_subtracted(&self.b.position);
        let va = self.a.velocity.to_added(&Vector2::new(
            -self.a.angular_velocity * ra.y,
            self.a.angular_velocity * ra.x,
        ));
        let vb = self.b.velocity.to_added(&Vector2::new(
            -self.b.angular_velocity * rb.y,
            self.b.angular_velocity * rb.x,
        ));

        let relative_velocity = va.to_subtracted(&vb);

        // Impulse along the normal
        let impulse_magnitude_along_normal = -(1.0 + elasticity)
            * relative_velocity.dot_product(&self.normal)
            / ((self.a.inverse_mass + self.b.inverse_mass)
                + (ra.cross_product(&self.normal).powi(2) * self.a.inverse_moment_of_inertia)
                + (rb.cross_product(&self.normal).powi(2) * self.b.inverse_moment_of_inertia));
        let impulse_along_normal = self.normal.to_scaled(impulse_magnitude_along_normal);

        // Impulse along the tangent
        let tangent = self.normal.create_perpendicular();
        let impulse_magnitude_along_tangent =
            angular_friction * -(1.0 + elasticity) * relative_velocity.dot_product(&tangent)
                / ((self.a.inverse_mass + self.b.inverse_mass)
                    + (ra.cross_product(&tangent).powi(2) * self.a.inverse_moment_of_inertia)
                    + (rb.cross_product(&tangent).powi(2) * self.b.inverse_moment_of_inertia));
        let impulse_along_tangent = tangent.to_scaled(impulse_magnitude_along_tangent);

        let result = impulse_along_normal.to_added(&impulse_along_tangent);

        self.a.apply_angular_impulse(&result, &ra);
        self.b.apply_angular_impulse(&result.to_scaled(-1.0), &rb);
    }

    fn resolve_penetration(&mut self) {
        let factor = self.depth / (self.a.inverse_mass + self.b.inverse_mass);

        let da = factor * self.a.inverse_mass;
        let db = factor * self.b.inverse_mass;

        self.a.position.subtract(&self.normal.to_scaled(da));
        self.b.position.add(&self.normal.to_scaled(db));

        self.a
            .shape
            .update_vertices(&self.a.position, self.a.rotation);
        self.b
            .shape
            .update_vertices(&self.b.position, self.b.rotation);
    }
}
