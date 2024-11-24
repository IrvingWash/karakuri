use std::cmp::Ordering;

use kmath::Vector2;

use crate::RigidBody;

use super::JointConstraint;

#[derive(Debug)]
pub enum Constraint {
    Joint(JointConstraint),
}

impl Constraint {
    #[inline]
    pub fn new_joint(a: &RigidBody, b: &RigidBody, anchor_point: &Vector2) -> Self {
        Self::Joint(JointConstraint::new(a, b, anchor_point))
    }

    #[inline]
    pub fn pre_solve(&mut self, rigid_bodies: &mut [RigidBody]) {
        match self {
            Self::Joint(joint_constraint) => {
                let mut a_index: Option<usize> = None;
                let mut b_index: Option<usize> = None;
                for (i, rb) in rigid_bodies.iter().enumerate() {
                    if a_index.is_none() && rb.id() == joint_constraint.a_id {
                        a_index = Some(i);
                    }
                    if b_index.is_none() && rb.id() == joint_constraint.b_id {
                        b_index = Some(i);
                    }

                    if a_index.is_some() && b_index.is_some() {
                        break;
                    }
                }

                if let (Some(a_index), Some(b_index)) = (a_index, b_index) {
                    let smaller_index = match a_index.cmp(&b_index) {
                        Ordering::Equal | Ordering::Less => a_index,
                        Ordering::Greater => b_index,
                    };

                    let (first, second) = rigid_bodies.split_at_mut(smaller_index + 1);

                    let (a, b);

                    if smaller_index == a_index {
                        b = &mut second[b_index - first.len()];
                        a = &mut first[a_index];
                    } else {
                        a = &mut second[a_index - first.len()];
                        b = &mut first[b_index];
                    }

                    joint_constraint.pre_solve(a, b);
                }
            }
        }
    }

    #[inline]
    pub fn solve(&mut self, rigid_bodies: &mut [RigidBody]) {
        match self {
            Self::Joint(joint_constraint) => {
                let mut a_index: Option<usize> = None;
                let mut b_index: Option<usize> = None;
                for (i, rb) in rigid_bodies.iter().enumerate() {
                    if a_index.is_none() && rb.id() == joint_constraint.a_id {
                        a_index = Some(i);
                    }
                    if b_index.is_none() && rb.id() == joint_constraint.b_id {
                        b_index = Some(i);
                    }

                    if a_index.is_some() && b_index.is_some() {
                        break;
                    }
                }

                if let (Some(a_index), Some(b_index)) = (a_index, b_index) {
                    let smaller_index = match a_index.cmp(&b_index) {
                        Ordering::Equal | Ordering::Less => a_index,
                        Ordering::Greater => b_index,
                    };

                    let (first, second) = rigid_bodies.split_at_mut(smaller_index + 1);

                    let (a, b);

                    if smaller_index == a_index {
                        b = &mut second[b_index - first.len()];
                        a = &mut first[a_index];
                    } else {
                        a = &mut second[a_index - first.len()];
                        b = &mut first[b_index];
                    }

                    joint_constraint.resolve(a, b);
                }
            }
        }
    }

    #[inline]
    pub fn post_solve(&self) {
        match self {
            Self::Joint(joint_constraint) => {
                joint_constraint.post_solve();
            }
        }
    }
}
