use crate::RigidBody;

#[allow(dead_code)]
pub struct Constraint<'a> {
    a: &'a mut RigidBody,
    b: &'a mut RigidBody,
}

impl<'a> Constraint<'a> {
    #[allow(dead_code)]
    #[inline]
    pub fn solve(&self) {}

    #[allow(dead_code)]
    fn inverse_matrix() {}
}
