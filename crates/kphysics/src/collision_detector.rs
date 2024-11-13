use core::f64;

use crate::{shapes::Polygon, Contact, RigidBody};

#[inline]
// TODO: Probably we shouldn't return ContactInformation here to optimize the process.
// Ask for contact information separately
pub fn are_colliding<'a>(a: &'a mut RigidBody, b: &'a mut RigidBody) -> Option<Contact<'a>> {
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

fn are_colliding_circles<'a>(a: &'a mut RigidBody, b: &'a mut RigidBody) -> Option<Contact<'a>> {
    let a_shape = a.shape.circle().unwrap();
    let b_shape = b.shape.circle().unwrap();

    let disposition = b.position.to_subtracted(&a.position);
    let radius_sum = a_shape.radius + b_shape.radius;

    if disposition.squared_magnitude() <= radius_sum.powi(2) {
        return Some(Contact::for_circles(a, b, &disposition));
    }

    None
}

fn are_colliding_polygons<'a>(a: &'a mut RigidBody, b: &'a mut RigidBody) -> Option<Contact<'a>> {
    let a_polygon = a.shape.polygon();
    let b_polygon = b.shape.polygon();

    let a_polygon = a_polygon.as_ref().unwrap();
    let b_polygon = b_polygon.as_ref().unwrap();

    let ab = find_minimum_separation(a_polygon, b_polygon);
    if ab >= 0.0 {
        return None;
    }

    let ba = find_minimum_separation(b_polygon, a_polygon);
    if ba >= 0.0 {
        return None;
    }

    Some(Contact::for_polygons(a, b))
}

#[allow(unused_variables)]
fn are_colliding_circle_and_polygon<'a>(
    circle: &'a RigidBody,
    rigid_body: &'a RigidBody,
) -> Option<Contact<'a>> {
    None
}

fn find_minimum_separation(a: &Polygon, b: &Polygon) -> f64 {
    let mut separation = f64::MIN;

    // TODO: Early return if separation was found?
    for (i, va) in a.world_vertices.iter().enumerate() {
        let normal = a.edge_at(i).create_perpendicular();

        let mut min_separation = f64::MAX;

        for vb in &b.world_vertices {
            let projection = vb.to_subtracted(va).dot_product(&normal);

            min_separation = min_separation.min(projection);
        }

        separation = separation.max(min_separation);
    }

    separation
}
