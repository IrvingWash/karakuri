use kmath::Vector2;
use kphysics::{shapes::Shape, RigidBody, RigidBodyParams};

#[derive(Debug)]
pub struct RigidBodyComponentParams {
    mass: f64,
    angular_friction: f64,
    bounciness: f64,
    can_be_rotated: bool,
}

impl Default for RigidBodyComponentParams {
    #[inline]
    fn default() -> Self {
        Self {
            mass: 1.0,
            angular_friction: 0.1,
            bounciness: 0.0,
            can_be_rotated: true,
        }
    }
}

#[derive(Debug)]
pub struct RigidBodyComponent {
    pub rigid_body: RigidBody,
}

impl RigidBodyComponent {
    #[inline]
    pub fn new(
        params: &RigidBodyComponentParams,
        position: &Vector2,
        rotation: f64,
        size: &Vector2,
    ) -> Self {
        let RigidBodyComponentParams {
            angular_friction,
            mass,
            bounciness,
            can_be_rotated,
        } = params;

        Self {
            rigid_body: RigidBody::new(RigidBodyParams {
                angular_friction: *angular_friction,
                mass: *mass,
                bounciness: *bounciness,
                can_be_rotated: *can_be_rotated,
                position: position.create_copy(),
                rotation,
                velocity: Vector2::ZERO,
                shape: Shape::new_rectangle(size.x, size.y),
            }),
        }
    }
}
