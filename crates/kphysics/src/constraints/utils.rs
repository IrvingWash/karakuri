use kmath::{Matrix, VectorN};

use crate::RigidBody;

#[inline]
pub fn inverse_mass_matrix(a: &RigidBody, b: &RigidBody) -> Matrix {
    let mut matrix = Matrix::new(6, 6);

    let data = matrix.data_mut();

    data[0][0] = a.inverse_mass();
    data[1][1] = a.inverse_mass();
    data[2][2] = a.inverse_moment_of_inertia();
    data[3][3] = b.inverse_mass();
    data[4][4] = b.inverse_mass();
    data[5][5] = b.inverse_moment_of_inertia();

    matrix
}

#[inline]
pub fn velocities(a: &RigidBody, b: &RigidBody) -> VectorN {
    VectorN::from_vec(&[
        a.velocity().x,
        a.velocity().y,
        a.angular_velocity(),
        b.velocity().x,
        b.velocity().y,
        b.angular_velocity(),
    ])
}

#[inline]
pub fn solve_gauss_seidel(lhs: &Matrix, rhs: &VectorN) -> VectorN {
    let n = rhs.len();

    let mut x = VectorN::new(n);

    for _ in 0..n {
        for i in 0..n {
            if lhs.data()[i][i] != 0.0 {
                x[i] += (rhs[i] / lhs.data()[i][i])
                    - (lhs.data()[i].dot_product(&x) / lhs.data()[i][i]);
            }
        }
    }

    x
}
