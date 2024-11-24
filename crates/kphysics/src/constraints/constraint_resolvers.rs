use kmath::{Matrix, Vector2, VectorN};

use crate::RigidBody;

#[inline]
pub fn resolve_joint_constraint(
    a: &mut RigidBody,
    b: &mut RigidBody,
    a_point: &Vector2,
    b_point: &Vector2,
) {
    let pa = a.local_to_world(a_point);
    let pb = b.local_to_world(b_point);

    let ra = pa.to_subtracted(a.position());
    let rb = pb.to_subtracted(b.position());

    let mut jacobian = Matrix::new(1, 6);

    let j1 = pa.to_subtracted(&pb).to_scaled(2.0);
    let j2 = ra.cross_product(&pa.to_subtracted(&pb)) * 2.0;
    let j3 = pb.to_subtracted(&pa).to_scaled(2.0);
    let j4 = rb.cross_product(&pb.to_subtracted(&pa)) * 2.0;

    let matrix_data = jacobian.data_mut();
    matrix_data[0][0] = j1.x;
    matrix_data[0][1] = j1.y;
    matrix_data[0][2] = j2;
    matrix_data[0][3] = j3.x;
    matrix_data[0][4] = j3.y;
    matrix_data[0][5] = j4;

    let velocities = velocities(a, b);
    let inverse_mass_matrix = inverse_mass_matrix(a, b);

    let jacobian_transposed = jacobian.to_transposed();

    let lhs = jacobian
        .to_multiplied_by_matrix(&inverse_mass_matrix)
        .to_multiplied_by_matrix(&jacobian_transposed);

    let rhs = jacobian.to_multiplied_by_vector(&velocities.to_scaled(-1.0));

    let lambda = solve_gauss_seidel(&lhs, &rhs);

    let impulses = jacobian_transposed.to_multiplied_by_vector(&lambda);

    a.apply_linear_impulse(&Vector2::new(impulses[0], impulses[1]));
    a.apply_angular_impulse(impulses[2]);
    b.apply_linear_impulse(&Vector2::new(impulses[3], impulses[4]));
    b.apply_angular_impulse(impulses[5]);
}

fn inverse_mass_matrix(a: &RigidBody, b: &RigidBody) -> Matrix {
    Matrix::from_data(&[
        VectorN::from_vec(&[a.inverse_mass(), 0.0, 0.0, 0.0, 0.0, 0.0]),
        VectorN::from_vec(&[0.0, a.inverse_mass(), 0.0, 0.0, 0.0, 0.0]),
        VectorN::from_vec(&[0.0, 0.0, a.inverse_moment_of_inertia(), 0.0, 0.0, 0.0]),
        VectorN::from_vec(&[0.0, 0.0, 0.0, b.inverse_mass(), 0.0, 0.0]),
        VectorN::from_vec(&[0.0, 0.0, 0.0, 0.0, b.inverse_mass(), 0.0]),
        VectorN::from_vec(&[0.0, 0.0, 0.0, 0.0, 0.0, b.inverse_moment_of_inertia()]),
    ])
}

fn velocities(a: &RigidBody, b: &RigidBody) -> VectorN {
    VectorN::from_vec(&[
        a.velocity().x,
        a.velocity().y,
        a.angular_velocity(),
        b.velocity().x,
        b.velocity().y,
        b.angular_velocity(),
    ])
}

fn solve_gauss_seidel(lhs: &Matrix, rhs: &VectorN) -> VectorN {
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
