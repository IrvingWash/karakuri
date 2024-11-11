use crate::RigidBody;

#[inline]
pub fn are_colliding(a: &RigidBody, b: &RigidBody) -> bool {
    if a.shape.is_circle() && b.shape.is_circle() {
        return are_colliding_circles(a, b);
    }

    if a.shape.is_polygon() && b.shape.is_polygon() {
        return are_colliding_polygons(a, b);
    }

    if a.shape.is_circle() && b.shape.is_polygon() {
        return are_colliding_circle_and_polygon(a, b);
    }

    if a.shape.is_polygon() && b.shape.is_circle() {
        return are_colliding_circle_and_polygon(b, a);
    }

    false
}

fn are_colliding_circles(a: &RigidBody, b: &RigidBody) -> bool {
    let a_shape = a.shape.circle().unwrap();
    let b_shape = b.shape.circle().unwrap();

    let disposition = b.position.to_subtracted(&a.position);
    let radius_sum = a_shape.radius + b_shape.radius;

    disposition.squared_magnitude() <= radius_sum.powi(2)
}

#[allow(unused_variables)]
fn are_colliding_polygons(a: &RigidBody, b: &RigidBody) -> bool {
    false
}

#[allow(unused_variables)]
fn are_colliding_circle_and_polygon(circle: &RigidBody, rigid_body: &RigidBody) -> bool {
    false
}
