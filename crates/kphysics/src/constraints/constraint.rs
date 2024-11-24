use super::{DistanceConstraint, PenetrationConstraint};
use crate::RigidBody;

#[allow(dead_code)]
pub enum Constraint<'a> {
    Distance(DistanceConstraint<'a>),
    Penetration(PenetrationConstraint<'a>),
}

#[allow(dead_code)]
impl<'a> Constraint<'a> {
    #[inline]
    pub fn new_distance(a: &'a mut RigidBody, b: &'a mut RigidBody) -> Self {
        Self::Distance(DistanceConstraint::new(a, b))
    }

    #[inline]
    pub fn new_penetration(a: &'a mut RigidBody, b: &'a mut RigidBody) -> Self {
        Self::Penetration(PenetrationConstraint::new(a, b))
    }

    #[inline]
    pub fn solve(&self) {
        match self {
            Constraint::Distance(distance_constraint) => distance_constraint.constraint().solve(),
            Constraint::Penetration(penetration_constraint) => {
                penetration_constraint.constraint().solve()
            }
        }
    }
}
