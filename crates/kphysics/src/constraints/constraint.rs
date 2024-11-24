use super::{JointConstraint, PenetrationConstraint};
use crate::RigidBody;

#[allow(dead_code)]
pub enum Constraint<'a> {
    Joint(JointConstraint<'a>),
    Penetration(PenetrationConstraint<'a>),
}

#[allow(dead_code)]
impl<'a> Constraint<'a> {
    #[inline]
    pub fn new_joint(a: &'a mut RigidBody, b: &'a mut RigidBody) -> Self {
        Self::Joint(JointConstraint::new(a, b))
    }

    #[inline]
    pub fn new_penetration(a: &'a mut RigidBody, b: &'a mut RigidBody) -> Self {
        Self::Penetration(PenetrationConstraint::new(a, b))
    }

    #[inline]
    pub fn solve(&self) {
        match self {
            Constraint::Joint(joint_constraint) => joint_constraint.constraint().solve(),
            Constraint::Penetration(penetration_constraint) => {
                penetration_constraint.constraint().solve()
            }
        }
    }
}
