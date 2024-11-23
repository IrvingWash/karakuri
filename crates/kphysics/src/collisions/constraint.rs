use kmath::{Matrix, VectorN};

use crate::RigidBody;

#[allow(dead_code)]
pub struct Constraint<'a> {
    a: &'a mut RigidBody,
    b: &'a mut RigidBody,
    jacobian: Matrix,
}

impl<'a> Constraint<'a> {
    #[allow(dead_code)]
    #[inline]
    pub fn solve(&self) {
        todo!()
    }

    #[allow(dead_code)]
    fn inverse_matrix() -> Matrix {
        todo!()
    }

    #[allow(dead_code)]
    fn velocities() -> VectorN {
        todo!()
    }
}
