use crate::RigidBody;
use kmath::Vector2;

#[derive(Debug)]
#[allow(dead_code)] // TODO: Remove asap
pub struct ContactInformation<'a> {
    a: &'a mut RigidBody,
    b: &'a mut RigidBody,

    pub normal: Vector2, // TODO: It's pub for debug
    depth: f64,

    pub start: Vector2, // TODO: It's pub for debug
    pub end: Vector2,   // TODO: It's pub for debug
}

impl<'a> ContactInformation<'a> {
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

    // TODO: It's kind of nice that we can resolve penetration from within the contact info,
    // so it can't be used for different set of bodies.
    // Although I don't like that much the fact that contact information is responsible for resolution
    #[inline]
    pub fn resolve_penetration(&mut self) {
        if self.a.is_static() && self.b.is_static() {
            return;
        }

        let factor = self.depth / (self.a.inverse_mass + self.b.inverse_mass);

        let da = factor * self.a.inverse_mass;
        let db = factor * self.b.inverse_mass;

        self.a.position.subtract(&self.normal.to_scaled(da));
        self.b.position.add(&self.normal.to_scaled(db));
    }
}
