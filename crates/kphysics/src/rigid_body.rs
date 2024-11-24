use kmath::Vector2;

use crate::shapes::Shape;

static mut NEXT_ID: usize = 0;

#[derive(Debug)]
pub struct RigidBodyParams {
    pub position: Vector2,
    pub mass: f64,
    pub shape: Shape,
    pub bounciness: f64,
    pub angular_friction: f64,
    pub rotation: f64,
    pub can_be_rotated: bool,
    pub velocity: Vector2,
}

impl Default for RigidBodyParams {
    #[inline]
    fn default() -> Self {
        Self {
            position: Vector2::ZERO,
            mass: 1.0,
            shape: Shape::new_rectangle(10.0, 10.0),
            bounciness: 0.0,
            angular_friction: 0.1,
            rotation: 0.0,
            can_be_rotated: false,
            velocity: Vector2::ZERO,
        }
    }
}

#[derive(Debug)]
pub struct RigidBody {
    id: usize,

    shape: Shape,

    // Linear motion
    position: Vector2,
    velocity: Vector2,
    accumulated_forces: Vector2,

    // Angular motion
    rotation: f64,
    angular_friction: f64,
    angular_velocity: f64,
    accumulated_torque: f64,

    bounciness: f64,
    mass: f64,
    inverse_mass: f64,
    #[allow(dead_code)]
    moment_of_inertia: f64,
    inverse_moment_of_inertia: f64,

    can_be_rotated: bool,

    is_static: bool,
}

impl RigidBody {
    #[inline]
    pub fn new(params: RigidBodyParams) -> Self {
        let RigidBodyParams {
            position,
            mass,
            shape,
            bounciness: restitution,
            angular_friction,
            rotation,
            can_be_rotated,
            velocity,
        } = params;

        let moment_of_inertia = shape.moment_of_inertia() * mass;

        let id = unsafe { NEXT_ID };

        unsafe {
            NEXT_ID += 1;
        }

        let mut s = Self {
            id,
            shape,
            position,
            velocity,
            accumulated_forces: Vector2::ZERO,
            rotation,
            angular_friction,
            angular_velocity: 0.0,
            accumulated_torque: 0.0,
            mass,
            inverse_mass: if mass != 0.0 { 1.0 / mass } else { 0.0 },
            moment_of_inertia,
            inverse_moment_of_inertia: if moment_of_inertia == 0.0 {
                0.0
            } else {
                1.0 / moment_of_inertia
            },
            bounciness: restitution,
            can_be_rotated,
            is_static: mass == 0.0,
        };

        // Using direct access to shape because self.update_vertices has early return for static bodies.
        s.shape.update_vertices(&s.position, s.rotation);

        s
    }

    #[inline]
    pub fn id(&self) -> usize {
        self.id
    }

    #[inline]
    pub fn is_static(&self) -> bool {
        self.is_static
    }

    #[inline]
    pub fn can_be_rotated(&self) -> bool {
        self.can_be_rotated
    }

    #[inline]
    pub fn inverse_moment_of_inertia(&self) -> f64 {
        self.inverse_moment_of_inertia
    }

    #[inline]
    pub fn inverse_mass(&self) -> f64 {
        self.inverse_mass
    }

    #[inline]
    pub fn mass(&self) -> f64 {
        self.mass
    }

    #[inline]
    pub fn bounciness(&self) -> f64 {
        self.bounciness
    }

    #[inline]
    pub fn rotation(&self) -> f64 {
        self.rotation
    }

    #[inline]
    pub fn angular_velocity(&self) -> f64 {
        self.angular_velocity
    }

    #[inline]
    pub fn angular_friction(&self) -> f64 {
        self.angular_friction
    }

    #[inline]
    pub fn velocity(&self) -> &Vector2 {
        &self.velocity
    }

    #[inline]
    pub fn position(&self) -> &Vector2 {
        &self.position
    }

    #[inline]
    pub fn position_mut(&mut self) -> &mut Vector2 {
        &mut self.position
    }

    #[inline]
    pub fn shape(&self) -> &Shape {
        &self.shape
    }

    #[inline]
    pub fn apply_force(&mut self, force: &Vector2) {
        if self.is_static {
            return;
        }

        self.accumulated_forces.add(force);
    }

    #[inline]
    pub fn apply_torque(&mut self, torque: f64) {
        if self.is_static || !self.can_be_rotated {
            return;
        }

        self.accumulated_torque += torque;
    }

    #[inline]
    pub fn apply_linear_impulse(&mut self, impulse: &Vector2) {
        if self.is_static {
            return;
        }

        self.velocity.add(&impulse.to_scaled(self.inverse_mass));
    }

    #[inline]
    pub fn apply_angular_impulse(&mut self, impulse: f64) {
        if self.is_static || !self.can_be_rotated {
            return;
        }

        self.angular_velocity += impulse * self.inverse_moment_of_inertia;
    }

    #[inline]
    pub fn apply_impulse_at_point(&mut self, impulse: &Vector2, r: &Vector2) {
        if self.is_static || !self.can_be_rotated {
            return;
        }

        self.velocity.add(&impulse.to_scaled(self.inverse_mass));
        self.angular_velocity += r.cross_product(impulse) * self.inverse_moment_of_inertia;
    }

    #[inline]
    pub fn integrate_forces(&mut self, delta_time: f64) {
        if self.is_static {
            return;
        }

        let acceleration = self.accumulated_forces.to_scaled(self.inverse_mass);
        self.velocity.add(&acceleration.to_scaled(delta_time));
        self.clear_forces();

        if self.can_be_rotated {
            let angular_acceleration = self.accumulated_torque * self.inverse_moment_of_inertia;
            self.angular_velocity += angular_acceleration * delta_time;
            self.clear_torque();
        }
    }

    #[inline]
    pub fn integrate_velocities(&mut self, delta_time: f64) {
        if self.is_static {
            return;
        }

        self.position.add(&self.velocity.to_scaled(delta_time));

        if self.can_be_rotated {
            self.rotation += self.angular_velocity * delta_time;
        }

        self.update_shape_vertices();
    }

    #[inline]
    pub fn update_shape_vertices(&mut self) {
        if self.is_static {
            return;
        }

        self.shape.update_vertices(&self.position, self.rotation);
    }

    #[inline]
    pub fn world_to_local(&self, point: &Vector2) -> Vector2 {
        let mut result = point.to_subtracted(&self.position);

        result.rotate(-self.rotation);

        result
    }

    #[inline]
    pub fn local_to_world(&self, point: &Vector2) -> Vector2 {
        let mut result = point.to_rotated(self.rotation);

        result.add(&self.position);

        result
    }

    fn clear_forces(&mut self) {
        self.accumulated_forces.reset();
    }

    fn clear_torque(&mut self) {
        self.accumulated_torque = 0.0;
    }
}

#[cfg(test)]
mod rigid_body_tests {
    use std::f64;

    use kmath::Vector2;

    use crate::{
        rigid_body::RigidBodyParams,
        shapes::{Circle, Shape},
    };

    use super::RigidBody;

    #[test]
    fn test_new() {
        // No restitution
        {
            let mass = 2.0;

            let rigid_body = RigidBody::new(RigidBodyParams {
                mass,
                shape: Shape::new_circle(10.0),
                bounciness: 1.0,
                ..Default::default()
            });

            assert_eq!(rigid_body.inverse_mass, 1.0 / mass);
            assert_eq!(
                rigid_body.inverse_moment_of_inertia,
                1.0 / (Circle::new(10.0).moment_of_inertia() * mass)
            );
            assert_eq!(rigid_body.bounciness, 1.0);
        }

        // Restitution
        {
            let mass = 2.0;

            let rigid_body = RigidBody::new(RigidBodyParams {
                mass,
                bounciness: 3.0,
                shape: Shape::new_circle(10.0),
                ..Default::default()
            });

            assert_eq!(rigid_body.bounciness, 3.0);
        }

        // Zero mass
        {
            let rigid_body = RigidBody::new(RigidBodyParams {
                mass: 0.0,
                ..Default::default()
            });

            assert_eq!(rigid_body.inverse_mass, 0.0);
            assert_eq!(rigid_body.inverse_moment_of_inertia, 0.0);
            assert!(rigid_body.is_static);
        }
    }

    #[test]
    fn test_force_application() {
        let mut rb = RigidBody::new(RigidBodyParams {
            position: Vector2::new(10.0, 10.0),
            mass: 1.5,
            shape: Shape::new_polygon(vec![
                Vector2::new(10.0, 5.0),
                Vector2::new(15.0, 15.0),
                Vector2::new(5.0, 15.0),
            ]),
            bounciness: 1.0,
            can_be_rotated: true,
            ..Default::default()
        });

        assert!(!rb.is_static);

        rb.apply_force(&Vector2::new(3.0, 3.0));
        rb.apply_force(&Vector2::new(5.0, 5.0));
        rb.apply_torque(3.0);
        rb.apply_torque(5.0);

        assert_eq!(rb.accumulated_forces, Vector2::new(8.0, 8.0));
        assert_eq!(rb.accumulated_torque, 8.0);

        rb.integrate_forces(2.0);
        rb.integrate_velocities(2.0);

        assert_eq!(rb.accumulated_forces, Vector2::ZERO);
        assert_eq!(rb.accumulated_torque, 0.0);

        assert_eq!(
            rb.position,
            Vector2::new(31.333333333333332, 31.333333333333332)
        );

        assert_eq!(
            rb.velocity,
            Vector2::new(10.666666666666666, 10.666666666666666)
        );

        assert_eq!(rb.rotation, 0.004266666666666667);

        rb.apply_linear_impulse(&Vector2::new(3.0, 5.0));

        assert_eq!(rb.velocity, Vector2::new(12.666666666666666, 14.0));
    }

    #[test]
    fn test_force_application_for_static() {
        let mut rb = RigidBody::new(RigidBodyParams {
            position: Vector2::new(10.0, 10.0),
            shape: Shape::new_polygon(vec![
                Vector2::new(10.0, 5.0),
                Vector2::new(15.0, 15.0),
                Vector2::new(5.0, 15.0),
            ]),
            bounciness: 1.0,
            mass: 0.0,
            ..Default::default()
        });

        assert!(rb.is_static);

        rb.apply_force(&Vector2::new(3.0, 3.0));
        rb.apply_force(&Vector2::new(5.0, 5.0));
        rb.apply_torque(3.0);
        rb.apply_torque(5.0);

        rb.integrate_forces(2.0);
        rb.integrate_velocities(2.0);

        assert_eq!(rb.position, Vector2::new(10.0, 10.0));
        assert_eq!(rb.rotation, 0.0);

        rb.apply_linear_impulse(&Vector2::new(10.0, 10.0));

        assert_eq!(rb.velocity, Vector2::ZERO);
    }

    #[test]
    fn test_local_to_world() {
        let rb = RigidBody::new(RigidBodyParams {
            position: Vector2::new(500.0, 300.0),
            shape: Shape::new_polygon(vec![
                Vector2::new(10.0, 5.0),
                Vector2::new(15.0, 15.0),
                Vector2::new(5.0, 15.0),
            ]),
            bounciness: 1.0,
            mass: 0.0,
            rotation: f64::consts::FRAC_2_PI,
            ..Default::default()
        });

        let point = Vector2::new(10.0, 10.0);

        let result = rb.local_to_world(&point);

        assert_eq!(
            result,
            Vector2 {
                x: 502.0962905970397,
                y: 313.98590596753616
            }
        );
    }

    #[test]
    fn test_world_to_local() {
        let rb = RigidBody::new(RigidBodyParams {
            position: Vector2::new(500.0, 300.0),
            shape: Shape::new_polygon(vec![
                Vector2::new(10.0, 5.0),
                Vector2::new(15.0, 15.0),
                Vector2::new(5.0, 15.0),
            ]),
            bounciness: 1.0,
            mass: 0.0,
            rotation: f64::consts::FRAC_2_PI,
            ..Default::default()
        });

        let point = Vector2::new(10.0, 10.0);

        let result = rb.world_to_local(&point);

        assert_eq!(
            result,
            Vector2 {
                x: -566.4132387043063,
                y: 58.10372639081322
            }
        );
    }
}
