use kmath::Vector2;

use crate::RigidBody;

use super::JointConstraintResolver;

#[derive(Debug)]
pub enum Constraint {
    Joint(JointConstraintDescriptor),
}

impl Constraint {
    pub fn new_joint(a_id: usize, b_id: usize, anchor_point: Vector2) -> Self {
        Self::Joint(JointConstraintDescriptor {
            a_id,
            b_id,
            anchor_point,
        })
    }

    pub fn solve(&self, rigid_bodies: &Vec<RigidBody>) {
        match self {
            Self::Joint(descriptor) => {
                let a = rigid_bodies.iter().find(|rb| rb.id() == descriptor.a_id);
                let b = rigid_bodies.iter().find(|rb| rb.id() == descriptor.b_id);

                if let (Some(a), Some(b)) = (a, b) {
                    let joint_resolver =
                        JointConstraintResolver::new(a, b, &descriptor.anchor_point);

                    joint_resolver.solve();
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct JointConstraintDescriptor {
    pub a_id: usize,
    pub b_id: usize,
    pub anchor_point: Vector2,
}
