use super::ConstraintBase;
use crate::RigidBody;
use kmath::Matrix;

pub struct PenetrationConstraint<'a> {
    constraint: ConstraintBase<'a>,
    jacobian: Matrix,
}

impl<'a> PenetrationConstraint<'a> {
    #[inline]
    pub fn new(a: &'a mut RigidBody, b: &'a mut RigidBody) -> PenetrationConstraint<'a> {
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
