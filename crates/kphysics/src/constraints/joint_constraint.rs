use kmath::{Matrix, Vector2, VectorN};

use crate::RigidBody;

#[derive(Debug)]
pub struct JointConstraint {
    pub a_id: usize,
    pub b_id: usize,
    a_point: Vector2,
    b_point: Vector2,

    // Cache
    jacobian: Matrix,
    jacobian_transposed: Matrix,
    inverse_mass_matrix: Matrix,
    lhs: Matrix,
    cached_lambda: Option<VectorN>,
}

impl JointConstraint {
    pub fn new(a: &RigidBody, b: &RigidBody, anchor_point: &Vector2) -> Self {
        Self {
            a_id: a.id(),
            b_id: b.id(),
            a_point: a.world_to_local(anchor_point),
            b_point: b.world_to_local(anchor_point),
            jacobian: Matrix::new(1, 6),
            jacobian_transposed: Matrix::new(6, 1),
            inverse_mass_matrix: inverse_mass_matrix(a, b),
            lhs: Matrix::new(1, 1),
            cached_lambda: None,
        }
    }

    #[inline]
    pub fn pre_solve(&mut self, a: &mut RigidBody, b: &mut RigidBody) {
        let pa = a.local_to_world(&self.a_point);
        let pb = b.local_to_world(&self.b_point);

        let ra = pa.to_subtracted(a.position());
        let rb = pb.to_subtracted(b.position());

        let pa_pb_diff = pa.to_subtracted(&pb);
        let pb_pa_diff = pb.to_subtracted(&pa);

        let j1 = pa_pb_diff.to_scaled(2.0);
        let j2 = ra.cross_product(&pa_pb_diff) * 2.0;
        let j3 = pb_pa_diff.to_scaled(2.0);
        let j4 = rb.cross_product(&pb_pa_diff) * 2.0;

        let matrix_data = self.jacobian.data_mut();
        matrix_data[0][0] = j1.x;
        matrix_data[0][1] = j1.y;
        matrix_data[0][2] = j2;
        matrix_data[0][3] = j3.x;
        matrix_data[0][4] = j3.y;
        matrix_data[0][5] = j4;

        self.jacobian_transposed = self.jacobian.to_transposed();

        self.lhs = self
            .jacobian
            .to_multiplied_by_matrix(&self.inverse_mass_matrix)
            .to_multiplied_by_matrix(&self.jacobian_transposed);

        match &self.cached_lambda {
            None => {}
            Some(cached_lambda) => {
                let impulses = self
                    .jacobian_transposed
                    .to_multiplied_by_vector(cached_lambda);

                a.apply_linear_impulse(&Vector2::new(impulses[0], impulses[1]));
                a.apply_angular_impulse(impulses[2]);
                b.apply_linear_impulse(&Vector2::new(impulses[3], impulses[4]));
                b.apply_angular_impulse(impulses[5]);
            }
        }
    }

    #[inline]
    pub fn resolve(&mut self, a: &mut RigidBody, b: &mut RigidBody) {
        let velocities = velocities(a, b);

        let rhs = self
            .jacobian
            .to_multiplied_by_vector(&velocities.to_scaled(-1.0));

        let lambda = solve_gauss_seidel(&self.lhs, &rhs);

        match &mut self.cached_lambda {
            None => self.cached_lambda = Some(lambda.clone()),
            Some(cached_lambda) => cached_lambda.add(&lambda),
        }

        let impulses = self.jacobian_transposed.to_multiplied_by_vector(&lambda);

        a.apply_linear_impulse(&Vector2::new(impulses[0], impulses[1]));
        a.apply_angular_impulse(impulses[2]);
        b.apply_linear_impulse(&Vector2::new(impulses[3], impulses[4]));
        b.apply_angular_impulse(impulses[5]);
    }

    #[inline]
    pub fn post_solve(&self) {}
}

fn inverse_mass_matrix(a: &RigidBody, b: &RigidBody) -> Matrix {
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

#[cfg(test)]
mod constraint_resolvers_tests {
    use kmath::Vector2;

    use crate::{constraints::Constraint, shapes::Shape, RigidBody, RigidBodyParams};

    #[test]
    fn test_joint_resolver() {
        let mut a = RigidBody::new(RigidBodyParams {
            shape: Shape::new_rectangle(30.0, 30.0),
            position: Vector2::new(250.0, 100.0),
            mass: 0.0,
            can_be_rotated: true,
            ..Default::default()
        });

        let mut b = RigidBody::new(RigidBodyParams {
            shape: Shape::new_rectangle(30.0, 30.0),
            position: Vector2::new(210.0, 100.0),
            mass: 1.0,
            can_be_rotated: true,
            ..Default::default()
        });

        let mut constraint = Constraint::new_joint(&a, &b, a.position());

        a.apply_force(&Vector2::new(0.0, 9.8));
        b.apply_force(&Vector2::new(0.0, 9.8));

        let delta_time = 2.0;

        a.integrate_forces(delta_time);
        b.integrate_forces(delta_time);

        match &mut constraint {
            Constraint::Joint(joint) => {
                joint.pre_solve(&mut a, &mut b);
                joint.resolve(&mut a, &mut b);
                joint.post_solve();
            }
        };

        a.integrate_velocities(delta_time);
        b.integrate_velocities(delta_time);

        a.integrate_forces(delta_time);
        b.integrate_forces(delta_time);

        match constraint {
            Constraint::Joint(mut joint) => {
                joint.pre_solve(&mut a, &mut b);
                joint.resolve(&mut a, &mut b);
                joint.post_solve();
            }
        };

        a.integrate_velocities(delta_time);
        b.integrate_velocities(delta_time);

        assert_eq!(a.position(), &Vector2::new(250.0, 100.0));
        assert_eq!(b.position(), &Vector2::new(210.0, 175.04));
    }
}
