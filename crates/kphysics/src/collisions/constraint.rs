use kmath::{Matrix, VectorN};

use crate::RigidBody;

#[allow(dead_code)]
pub enum Constraint<'a> {
    Distance(DistanceConstraint<'a>),
    Penetration(PenetrationConstraint<'a>),
}

#[allow(dead_code)]
impl<'a> Constraint<'a> {
    #[inline]
    pub fn distance(a: &'a mut RigidBody, b: &'a mut RigidBody) -> Self {
        Self::Distance(DistanceConstraint::new(a, b))
    }

    #[inline]
    pub fn penetration(a: &'a mut RigidBody, b: &'a mut RigidBody) -> Self {
        Self::Penetration(PenetrationConstraint::new(a, b))
    }

    #[inline]
    pub fn solve(&self) {
        match self {
            Constraint::Distance(distance_constraint) => distance_constraint.constraint.solve(),
            Constraint::Penetration(penetration_constraint) => {
                penetration_constraint.constraint.solve()
            }
        }
    }
}

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
}

#[allow(dead_code)]
pub struct PenetrationConstraint<'a> {
    constraint: ConstraintBase<'a>,
    jacobian: Matrix,
}

#[allow(dead_code)]
impl<'a> PenetrationConstraint<'a> {
    #[inline]
    pub fn new(a: &'a mut RigidBody, b: &'a mut RigidBody) -> PenetrationConstraint<'a> {
        Self {
            constraint: ConstraintBase::new(a, b),
            jacobian: Matrix::new(0, 0),
        }
    }
}

#[allow(dead_code)]
struct ConstraintBase<'a> {
    a: &'a mut RigidBody,
    b: &'a mut RigidBody,

    // Cache
    inverse_mass_matrix: Matrix,
    velocities: VectorN,
}

impl<'a> ConstraintBase<'a> {
    #[allow(dead_code)]
    #[inline]
    pub fn new(a: &'a mut RigidBody, b: &'a mut RigidBody) -> Self {
        Self {
            inverse_mass_matrix: Matrix::from_data(&[
                VectorN::from_vec(&[a.inverse_mass, 0.0, 0.0, 0.0, 0.0, 0.0]),
                VectorN::from_vec(&[0.0, a.inverse_mass, 0.0, 0.0, 0.0, 0.0]),
                VectorN::from_vec(&[0.0, 0.0, a.inverse_moment_of_inertia, 0.0, 0.0, 0.0]),
                VectorN::from_vec(&[0.0, 0.0, 0.0, b.inverse_mass, 0.0, 0.0]),
                VectorN::from_vec(&[0.0, 0.0, 0.0, 0.0, b.inverse_mass, 0.0]),
                VectorN::from_vec(&[0.0, 0.0, 0.0, 0.0, 0.0, b.inverse_moment_of_inertia]),
            ]),
            velocities: VectorN::from_vec(&[
                a.velocity.x,
                a.velocity.y,
                a.angular_velocity,
                b.velocity.x,
                b.velocity.y,
                b.angular_velocity,
            ]),

            a,
            b,
        }
    }

    #[allow(dead_code)]
    #[inline]
    pub fn solve(&self) {
        todo!()
    }

    #[allow(dead_code)]
    fn inverse_mass_matrix(&self) -> &Matrix {
        &self.inverse_mass_matrix
    }

    #[allow(dead_code)]
    fn velocities(&self) -> &VectorN {
        &self.velocities
    }
}
