use std::cmp::Ordering;

use kmath::Vector2;

use crate::RigidBody;

use super::{JointConstraint, PenetrationConstraint};

#[derive(Debug)]
pub enum Constraint {
    Joint(JointConstraint),
    Penetration(PenetrationConstraint),
}

impl Constraint {
    #[inline]
    pub fn new_joint(a: &RigidBody, b: &RigidBody, anchor_point: &Vector2) -> Self {
        Self::Joint(JointConstraint::new(a, b, anchor_point))
    }

    #[inline]
    pub fn new_penetration(
        a: &RigidBody,
        b: &RigidBody,
        a_collision_point: &Vector2,
        b_collision_point: &Vector2,
        normal: &Vector2,
    ) -> Self {
        Self::Penetration(PenetrationConstraint::new(
            a,
            b,
            a_collision_point,
            b_collision_point,
            normal,
        ))
    }

    #[inline]
    pub fn pre_solve(&mut self, rigid_bodies: &mut [RigidBody], delta_time: f64) {
        match self {
            Self::Joint(joint_constraint) => {
                let (a_index, b_index) =
                    Self::get_indexes(rigid_bodies, joint_constraint.a_id, joint_constraint.b_id);

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

                    joint_constraint.pre_solve(a, b, delta_time);
                }
            }
            Self::Penetration(penetration_constraint) => {
                // TODO: This indexing sucks.
                // Try to come up with something else.
                // Maybe make the constraint a part of RigidBody
                let (a_index, b_index) = Self::get_indexes(
                    rigid_bodies,
                    penetration_constraint.a_id,
                    penetration_constraint.b_id,
                );

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

                    penetration_constraint.pre_solve(a, b, delta_time);
                }
            }
        }
    }

    #[inline]
    pub fn solve(&mut self, rigid_bodies: &mut [RigidBody]) {
        match self {
            Self::Joint(joint_constraint) => {
                let (a_index, b_index) =
                    Self::get_indexes(rigid_bodies, joint_constraint.a_id, joint_constraint.b_id);

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
            Self::Penetration(penetration_constraint) => {
                let (a_index, b_index) = Self::get_indexes(
                    rigid_bodies,
                    penetration_constraint.a_id,
                    penetration_constraint.b_id,
                );

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

                    penetration_constraint.resolve(a, b);
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
            Self::Penetration(penetration_constraint) => {
                penetration_constraint.post_solve();
            }
        }
    }

    fn get_indexes(
        rigid_bodies: &[RigidBody],
        a_id: usize,
        b_id: usize,
    ) -> (Option<usize>, Option<usize>) {
        let mut a_index: Option<usize> = None;
        let mut b_index: Option<usize> = None;

        for (i, rb) in rigid_bodies.iter().enumerate() {
            if a_index.is_none() && rb.id() == a_id {
                a_index = Some(i);
            }
            if b_index.is_none() && rb.id() == b_id {
                b_index = Some(i);
            }

            if a_index.is_some() && b_index.is_some() {
                break;
            }
        }

        (a_index, b_index)
    }
}
