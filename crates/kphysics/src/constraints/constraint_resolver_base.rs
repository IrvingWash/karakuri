use crate::RigidBody;
use kmath::{Matrix, Vector2, VectorN};

pub struct ConstraintResolverBase<'a> {
    a: &'a RigidBody,
    b: &'a RigidBody,

    a_point: Vector2,
    b_point: Vector2,

    // Cache
    inverse_mass_matrix: Matrix,
    velocities: VectorN,
}

impl<'a> ConstraintResolverBase<'a> {
    #[inline]
    pub fn new(a: &'a RigidBody, b: &'a RigidBody, a_point: Vector2, b_point: Vector2) -> Self {
        Self {
            a,
            b,

            a_point,
            b_point,

            inverse_mass_matrix: Matrix::from_data(&[
                VectorN::from_vec(&[a.inverse_mass(), 0.0, 0.0, 0.0, 0.0, 0.0]),
                VectorN::from_vec(&[0.0, a.inverse_mass(), 0.0, 0.0, 0.0, 0.0]),
                VectorN::from_vec(&[0.0, 0.0, a.inverse_moment_of_inertia(), 0.0, 0.0, 0.0]),
                VectorN::from_vec(&[0.0, 0.0, 0.0, b.inverse_mass(), 0.0, 0.0]),
                VectorN::from_vec(&[0.0, 0.0, 0.0, 0.0, b.inverse_mass(), 0.0]),
                VectorN::from_vec(&[0.0, 0.0, 0.0, 0.0, 0.0, b.inverse_moment_of_inertia()]),
            ]),
            velocities: VectorN::from_vec(&[
                a.velocity().x,
                a.velocity().y,
                a.angular_velocity(),
                b.velocity().x,
                b.velocity().y,
                b.angular_velocity(),
            ]),
        }
    }

    #[inline]
    pub fn solve(&self) {
        todo!()
    }

    fn inverse_mass_matrix(&self) -> &Matrix {
        &self.inverse_mass_matrix
    }

    fn velocities(&self) -> &VectorN {
        &self.velocities
    }
}
