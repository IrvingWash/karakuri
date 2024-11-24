use crate::RigidBody;
use kmath::{Matrix, VectorN};

#[allow(dead_code)]
pub struct ConstraintBase<'a> {
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
                VectorN::from_vec(&[a.inverse_mass(), 0.0, 0.0, 0.0, 0.0, 0.0]),
                VectorN::from_vec(&[0.0, a.inverse_mass(), 0.0, 0.0, 0.0, 0.0]),
                VectorN::from_vec(&[0.0, 0.0, a.inverse_moment_of_inertia(), 0.0, 0.0, 0.0]),
                VectorN::from_vec(&[0.0, 0.0, 0.0, b.inverse_mass(), 0.0, 0.0]),
                VectorN::from_vec(&[0.0, 0.0, 0.0, 0.0, b.inverse_mass(), 0.0]),
                VectorN::from_vec(&[0.0, 0.0, 0.0, 0.0, 0.0, b.inverse_moment_of_inertia()]),
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
