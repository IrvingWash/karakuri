use super::ConstraintBase;
use crate::RigidBody;
use kmath::Matrix;

#[allow(dead_code)]
pub struct DistanceConstraint<'a> {
    constraint: ConstraintBase<'a>,
    jacobian: Matrix,
}

#[allow(dead_code)]
impl<'a> DistanceConstraint<'a> {
    #[inline]
    pub fn new(a: &'a mut RigidBody, b: &'a mut RigidBody) -> DistanceConstraint<'a> {
        Self {
            constraint: ConstraintBase::new(a, b),
            jacobian: Matrix::new(0, 0),
        }
    }

    #[inline]
    pub fn constraint(&self) -> &ConstraintBase<'a> {
        &self.constraint
    }
}
