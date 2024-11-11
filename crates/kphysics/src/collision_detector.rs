use crate::{ContactInformation, RigidBody};

#[inline]
// TODO: Probably we shouldn't return ContactInformation here to optimize the process.
// Ask for contact information separately
pub fn are_colliding<'a>(a: &'a RigidBody, b: &'a RigidBody) -> Option<ContactInformation<'a>> {
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

    None
}

fn are_colliding_circles<'a>(a: &'a RigidBody, b: &'a RigidBody) -> Option<ContactInformation<'a>> {
    let a_shape = a.shape.circle().unwrap();
    let b_shape = b.shape.circle().unwrap();

    let disposition = b.position.to_subtracted(&a.position);
    let radius_sum = a_shape.radius + b_shape.radius;

    if disposition.squared_magnitude() <= radius_sum.powi(2) {
        return Some(ContactInformation::for_circles(a, b, &disposition));
    }

    None
}

#[allow(unused_variables)]
fn are_colliding_polygons<'a>(
    a: &'a RigidBody,
    b: &'a RigidBody,
) -> Option<ContactInformation<'a>> {
    None
}

#[allow(unused_variables)]
fn are_colliding_circle_and_polygon<'a>(
    circle: &'a RigidBody,
    rigid_body: &'a RigidBody,
) -> Option<ContactInformation<'a>> {
    None
}
