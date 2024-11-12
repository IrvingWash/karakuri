use crate::RigidBody;
use kmath::Vector2;

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

        let start = b
            .position
            .to_subtracted(&normal.to_scaled(b.shape.circle().unwrap().radius));

        let end = a
            .position
            .to_added(&normal.to_scaled(a.shape.circle().unwrap().radius));

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
    pub fn for_polygons(a: &'a mut RigidBody, b: &'a mut RigidBody) -> Self {
        Self {
            a,
            b,
            // TODO
            normal: Vector2::ZERO,
            depth: 0.0,
            start: Vector2::ZERO,
            end: Vector2::ZERO,
        }
    }

    #[inline]
    pub fn resolve_collision(&mut self) {
        if self.a.is_static() && self.b.is_static() {
            return;
        }

        self.resolve_penetration();

        let elasticity = self.a.restitution.min(self.b.restitution);

        let relative_velocity = self.a.velocity.to_subtracted(&self.b.velocity);

        let impulse_magnitude = -(1.0 + elasticity) * relative_velocity.dot_product(&self.normal)
            / (self.a.inverse_mass + self.b.inverse_mass);

        let result = self.normal.to_scaled(impulse_magnitude);

        self.a.apply_impulse(&result);
        self.b.apply_impulse(&result.to_scaled(-1.0));
    }

    fn resolve_penetration(&mut self) {
        let factor = self.depth / (self.a.inverse_mass + self.b.inverse_mass);

        let da = factor * self.a.inverse_mass;
        let db = factor * self.b.inverse_mass;

        self.a.position.subtract(&self.normal.to_scaled(da));
        self.b.position.add(&self.normal.to_scaled(db));
    }
}
