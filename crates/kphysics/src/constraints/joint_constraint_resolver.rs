use crate::RigidBody;
use kmath::{Matrix, Vector2};

use super::ConstraintResolverBase;

pub struct JointConstraintResolver<'a> {
    resolver: ConstraintResolverBase<'a>,
    jacobian: Matrix,
}

impl<'a> JointConstraintResolver<'a> {
    #[inline]
    pub fn new(
        a: &'a RigidBody,
        b: &'a RigidBody,
        anchor_point: &'a Vector2,
    ) -> JointConstraintResolver<'a> {
        let a_point = a.world_to_local(&anchor_point);
        let b_point = b.world_to_local(&anchor_point);

        Self {
            resolver: ConstraintResolverBase::new(a, b, a_point, b_point),
            jacobian: Matrix::new(1, 6),
        }
    }

    #[inline]
    pub fn solve(&self) {
        self.resolver.solve();
    }
}
