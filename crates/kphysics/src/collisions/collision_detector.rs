use core::f64;

use kmath::Vector2;

use crate::{shapes::Polygon, RigidBody};

use super::{
    errors::{panic_checked_circle_unwrap, panic_checked_polygon_unwrap},
    Contact, SeparationInfo,
};

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
    let a_shape = a
        .shape
        .circle()
        .unwrap_or_else(|| panic_checked_circle_unwrap());
    let b_shape = b
        .shape
        .circle()
        .unwrap_or_else(|| panic_checked_circle_unwrap());

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

    let a_polygon = a_polygon
        .as_ref()
        .unwrap_or_else(|| panic_checked_polygon_unwrap());
    let b_polygon = b_polygon
        .as_ref()
        .unwrap_or_else(|| panic_checked_polygon_unwrap());

    let ab_separation_info = find_minimum_separation(a_polygon, b_polygon);
    if ab_separation_info.separation >= 0.0 {
        return None;
    }

    let ba_separation_info = find_minimum_separation(b_polygon, a_polygon);
    if ba_separation_info.separation >= 0.0 {
        return None;
    }

    Some(Contact::for_polygons(
        a,
        b,
        ab_separation_info,
        ba_separation_info,
    ))
}

#[allow(unused_variables)]
fn are_colliding_circle_and_polygon<'a>(
    circle: &'a RigidBody,
    rigid_body: &'a RigidBody,
) -> Option<Contact<'a>> {
    None
}

fn find_minimum_separation(a: &Polygon, b: &Polygon) -> SeparationInfo {
    let mut separation = f64::MIN;
    let mut separation_axis = Vector2::ZERO;
    let mut point = Vector2::ZERO;

    for (i, va) in a.world_vertices.iter().enumerate() {
        let edge = a.edge_at(i);

        let normal = edge.create_perpendicular();

        let mut min_separation = f64::MAX;
        let mut min_vertex = Vector2::ZERO;

        for vb in &b.world_vertices {
            let projection = vb.to_subtracted(va).dot_product(&normal);

            if projection < min_separation {
                min_separation = projection;
                min_vertex.set(vb);
            };
        }

        if min_separation > separation {
            separation = min_separation;
            separation_axis = edge;
            point.set(&min_vertex);
        }
    }

    SeparationInfo {
        point,
        separation,
        separation_axis,
    }
}
