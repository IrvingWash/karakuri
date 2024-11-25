use kmath::{Matrix, Vector2, VectorN};

use crate::RigidBody;

use super::utils;

#[derive(Debug)]
pub struct PenetrationConstraint {
    pub a_id: usize,
    pub b_id: usize,

    a_point: Vector2,
    b_point: Vector2,
    normal: Vector2,
    angular_friction: f64,
    bias: f64,

    // Cache
    jacobian: Matrix,
    jacobian_transposed: Matrix,
    inverse_mass_matrix: Matrix,
    lhs: Matrix,
    cached_lambda: VectorN,
}

impl PenetrationConstraint {
    #[inline]
    pub fn new(
        a: &RigidBody,
        b: &RigidBody,
        a_collision_point: &Vector2,
        b_collision_point: &Vector2,
        normal: &Vector2,
    ) -> Self {
        Self {
            a_id: a.id(),
            b_id: b.id(),

            a_point: a.world_to_local(a_collision_point),
            b_point: b.world_to_local(b_collision_point),
            normal: a.world_to_local(normal),
            angular_friction: 0.0,
            bias: 0.0,

            jacobian: Matrix::new(2, 6),
            jacobian_transposed: Matrix::new(6, 2),
            inverse_mass_matrix: utils::inverse_mass_matrix(a, b),
            lhs: Matrix::new(1, 1),
            cached_lambda: VectorN::new(2),
        }
    }

    pub fn pre_solve(&mut self, a: &mut RigidBody, b: &mut RigidBody, delta_time: f64) {
        let pa = a.local_to_world(&self.a_point);
        let pb = b.local_to_world(&self.b_point);
        let n = a.local_to_world(&self.normal);

        let ra = pa.to_subtracted(a.position());
        let rb = pb.to_subtracted(b.position());

        let j1 = n.to_scaled(-1.0);
        let j2 = ra.to_scaled(-1.0).cross_product(&n);
        let j3 = &n;
        let j4 = rb.cross_product(&n);

        let matrix_data = self.jacobian.data_mut();
        matrix_data[0][0] = j1.x;
        matrix_data[0][1] = j1.y;
        matrix_data[0][2] = j2;
        matrix_data[0][3] = j3.x;
        matrix_data[0][4] = j3.y;
        matrix_data[0][5] = j4;

        self.angular_friction = a.angular_friction().max(b.angular_friction());
        if self.angular_friction > 0.0 {
            let t = n.create_perpendicular();

            matrix_data[1][0] = -t.x;
            matrix_data[1][1] = -t.y;
            matrix_data[1][2] = ra.to_scaled(-1.0).cross_product(&t);
            matrix_data[1][3] = t.x;
            matrix_data[1][4] = t.y;
            matrix_data[1][5] = rb.cross_product(&t);
        }

        self.jacobian_transposed = self.jacobian.to_transposed();

        self.lhs = self
            .jacobian
            .to_multiplied_by_matrix(&self.inverse_mass_matrix)
            .to_multiplied_by_matrix(&self.jacobian_transposed);

        let impulses = self
            .jacobian_transposed
            .to_multiplied_by_vector(&self.cached_lambda);

        a.apply_linear_impulse(&Vector2::new(impulses[0], impulses[1]));
        a.apply_angular_impulse(impulses[2]);
        b.apply_linear_impulse(&Vector2::new(impulses[3], impulses[4]));
        b.apply_angular_impulse(impulses[5]);

        let beta = 0.2;
        let c = pb.to_subtracted(&pa).dot_product(&n.to_scaled(-1.0));
        self.bias = beta / delta_time * c;
    }

    pub fn resolve(&mut self, a: &mut RigidBody, b: &mut RigidBody) {
        let velocities = utils::velocities(a, b);

        let mut rhs = self
            .jacobian
            .to_multiplied_by_vector(&velocities.to_scaled(-1.0));

        rhs[0] -= self.bias;

        let lambda = utils::solve_gauss_seidel(&self.lhs, &rhs);

        let previous_cached_lambda = self.cached_lambda.create_copy();
        self.cached_lambda.add(&lambda);
        self.cached_lambda[0] = if self.cached_lambda[0] < 0.0 {
            0.0
        } else {
            self.cached_lambda[0]
        };

        let resulting_lambda = self.cached_lambda.to_subtracted(&previous_cached_lambda);

        let impulses = self
            .jacobian_transposed
            .to_multiplied_by_vector(&resulting_lambda);

        a.apply_linear_impulse(&Vector2::new(impulses[0], impulses[1]));
        a.apply_angular_impulse(impulses[2]);
        b.apply_linear_impulse(&Vector2::new(impulses[3], impulses[4]));
        b.apply_angular_impulse(impulses[5]);
    }

    pub fn post_solve(&self) {}
}
