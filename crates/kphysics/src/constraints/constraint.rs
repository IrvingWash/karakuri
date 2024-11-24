use std::cmp::Ordering;

use kmath::Vector2;

use crate::RigidBody;

use super::constraint_resolvers::resolve_joint_constraint;

#[derive(Debug)]
pub enum Constraint {
    Joint(JointConstraintDescriptor),
}

impl Constraint {
    #[inline]
    pub fn new_joint(a: &RigidBody, b: &RigidBody, anchor_point: &Vector2) -> Self {
        Self::Joint(JointConstraintDescriptor {
            a_id: a.id(),
            b_id: b.id(),
            a_point: a.world_to_local(anchor_point),
            b_point: b.world_to_local(anchor_point),
        })
    }

    #[inline]
    pub fn solve(&self, rigid_bodies: &mut [RigidBody]) {
        match self {
            Self::Joint(descriptor) => {
                let mut a_index: Option<usize> = None;
                let mut b_index: Option<usize> = None;
                for (i, rb) in rigid_bodies.iter().enumerate() {
                    if a_index.is_none() && rb.id() == descriptor.a_id {
                        a_index = Some(i);
                    }
                    if b_index.is_none() && rb.id() == descriptor.b_id {
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

                    resolve_joint_constraint(a, b, &descriptor.a_point, &descriptor.b_point);
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct JointConstraintDescriptor {
    pub a_id: usize,
    pub b_id: usize,
    a_point: Vector2,
    b_point: Vector2,
}
