use kmath::Vector2;

use crate::shapes::Shape;

#[derive(Debug)]
pub struct RigidBodyParams {
    pub position: Vector2,
    pub mass: f64,
    pub shape: Shape,
    pub bounciness: f64,
    pub angular_friction: f64,
    pub rotation: f64,
    pub can_be_rotated: bool,
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
        }
    }
}

// TODO: Maybe we should have three types of rigid bodies based on the shape?
#[derive(Debug)]
pub struct RigidBody {
    pub shape: Shape,

    // Linear motion
    pub position: Vector2,
    pub velocity: Vector2,
    pub accumulated_forces: Vector2,

    // Angular motion
    pub rotation: f64,
    pub angular_friction: f64,
    pub angular_velocity: f64,
    pub accumulated_torque: f64,

    pub bounciness: f64,
    pub mass: f64,
    pub inverse_mass: f64,
    pub moment_of_inertia: f64,
    pub inverse_moment_of_inertia: f64,

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
        } = params;

        let moment_of_inertia = shape.moment_of_inertia() * mass;

        let mut s = Self {
            shape,
            position,
            velocity: Vector2::ZERO,
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

        s.shape.update_vertices(&s.position, s.rotation);

        s
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
    pub fn apply_force(&mut self, force: &Vector2) {
        self.accumulated_forces.add(force);
    }

    #[inline]
    pub fn apply_torque(&mut self, torque: f64) {
        self.accumulated_torque += torque;
    }

    #[inline]
    pub fn apply_impulse(&mut self, impulse: &Vector2) {
        if self.is_static {
            return;
        }

        self.velocity.add(&impulse.to_scaled(self.inverse_mass));
    }

    #[inline]
    pub fn apply_angular_impulse(&mut self, impulse: &Vector2, r: &Vector2) {
        if self.is_static {
            return;
        }

        self.velocity.add(&impulse.to_scaled(self.inverse_mass));
        self.angular_velocity += r.cross_product(impulse) * self.inverse_moment_of_inertia;
    }

    #[inline]
    pub fn update(&mut self, delta_time: f64) {
        self.integrate_linear(delta_time);
        self.integrate_angular(delta_time);
        self.update_vertices();
    }

    fn integrate_linear(&mut self, delta_time: f64) {
        if self.is_static {
            return;
        }

        let acceleration = self.accumulated_forces.to_scaled(self.inverse_mass);

        self.velocity.add(&acceleration.to_scaled(delta_time));

        self.position.add(&self.velocity.to_scaled(delta_time));

        self.clear_forces();
    }

    fn integrate_angular(&mut self, delta_time: f64) {
        if self.is_static {
            return;
        }

        let angular_acceleration = self.accumulated_torque * self.inverse_moment_of_inertia;

        self.angular_velocity += angular_acceleration * delta_time;

        self.rotation += self.angular_velocity * delta_time;

        self.clear_torque();
    }

    fn update_vertices(&mut self) {
        if self.is_static {
            return;
        }

        self.shape.update_vertices(&self.position, self.rotation);
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
            ..Default::default()
        });

        assert!(!rb.is_static);

        rb.apply_force(&Vector2::new(3.0, 3.0));
        rb.apply_force(&Vector2::new(5.0, 5.0));
        rb.apply_torque(3.0);
        rb.apply_torque(5.0);

        assert_eq!(rb.accumulated_forces, Vector2::new(8.0, 8.0));
        assert_eq!(rb.accumulated_torque, 8.0);

        rb.update(2.0);

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

        rb.apply_impulse(&Vector2::new(3.0, 5.0));

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

        rb.update(2.0);

        assert_eq!(rb.position, Vector2::new(10.0, 10.0));
        assert_eq!(rb.rotation, 0.0);

        rb.apply_impulse(&Vector2::new(10.0, 10.0));

        assert_eq!(rb.velocity, Vector2::ZERO);
    }
}
