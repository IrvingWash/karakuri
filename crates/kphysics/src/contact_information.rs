use crate::RigidBody;
use kmath::Vector2;

#[derive(Debug)]
#[allow(dead_code)] // TODO: Remove asap
pub struct ContactInformation<'a> {
    a: &'a RigidBody,
    b: &'a RigidBody,

    pub normal: Vector2, // TODO: It's pub for debug
    depth: f64,

    pub start: Vector2, // TODO: It's pub for debug
    pub end: Vector2,   // TODO: It's pub for debug
}

impl<'a> ContactInformation<'a> {
    #[inline]
    pub fn for_circles(a: &'a RigidBody, b: &'a RigidBody, disposition: &Vector2) -> Self {
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
}
