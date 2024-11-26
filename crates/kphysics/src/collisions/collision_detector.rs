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
pub fn are_colliding<'a>(a: &'a RigidBody, b: &'a RigidBody) -> Option<Vec<Contact<'a>>> {
    if a.shape().is_circle() && b.shape().is_circle() {
        return are_colliding_circles(a, b);
    }

    if a.shape().is_polygon() && b.shape().is_polygon() {
        return are_colliding_polygons(a, b);
    }

    if a.shape().is_circle() && b.shape().is_polygon() {
        return are_colliding_circle_and_polygon(a, b);
    }

    if a.shape().is_polygon() && b.shape().is_circle() {
        return are_colliding_circle_and_polygon(b, a);
    }

    None
}

fn are_colliding_circles<'a>(a: &'a RigidBody, b: &'a RigidBody) -> Option<Vec<Contact<'a>>> {
    let a_shape = a
        .shape()
        .circle()
        .unwrap_or_else(|| panic_checked_circle_unwrap());
    let b_shape = b
        .shape()
        .circle()
        .unwrap_or_else(|| panic_checked_circle_unwrap());

    let disposition = b.position().to_subtracted(a.position());
    let radius_sum = a_shape.radius() + b_shape.radius();

    if disposition.squared_magnitude() <= radius_sum.powi(2) {
        return Some(vec![Contact::for_circles(a, b, &disposition)]);
    }

    None
}

fn are_colliding_polygons<'a>(a: &'a RigidBody, b: &'a RigidBody) -> Option<Vec<Contact<'a>>> {
    let a_polygon = a
        .shape()
        .polygon()
        .unwrap_or_else(|| panic_checked_polygon_unwrap());
    let b_polygon = b
        .shape()
        .polygon()
        .unwrap_or_else(|| panic_checked_polygon_unwrap());

    let ab_separation_info = find_minimum_separation(a_polygon, b_polygon);
    if ab_separation_info.separation >= 0.0 {
        return None;
    }

    let ba_separation_info = find_minimum_separation(b_polygon, a_polygon);
    if ba_separation_info.separation >= 0.0 {
        return None;
    }

    let (reference_shape, incident_shape, reference_edge_index) =
        if ab_separation_info.separation > ba_separation_info.separation {
            (
                a.shape()
                    .polygon()
                    .unwrap_or_else(|| panic_checked_polygon_unwrap()),
                b.shape()
                    .polygon()
                    .unwrap_or_else(|| panic_checked_polygon_unwrap()),
                ab_separation_info.reference_edge_index,
            )
        } else {
            (
                b.shape()
                    .polygon()
                    .unwrap_or_else(|| panic_checked_polygon_unwrap()),
                a.shape()
                    .polygon()
                    .unwrap_or_else(|| panic_checked_polygon_unwrap()),
                ba_separation_info.reference_edge_index,
            )
        };

    let reference_edge = reference_shape.edge_at(reference_edge_index);

    let reference_edge_perpendicular = reference_edge.create_perpendicular();

    let incident_edge_index =
        incident_shape.find_incident_edge_index(&reference_edge_perpendicular);
    let incident_edge_next_index =
        (incident_edge_index + 1) % incident_shape.world_vertices().len();

    let v0 = incident_shape.world_vertices()[incident_edge_index].clone();
    let v1 = incident_shape.world_vertices()[incident_edge_next_index].clone();

    let mut contact_points = vec![v0, v1];
    let mut clipped_points = contact_points.clone();

    for i in 0..reference_shape.world_vertices().len() {
        if i == reference_edge_index {
            continue;
        }

        let c0 = &reference_shape.world_vertices()[i];
        let c1 =
            &reference_shape.world_vertices()[(i + 1) % reference_shape.world_vertices().len()];

        let clipped_count =
            reference_shape.clip_segment_to_line(&contact_points, &mut clipped_points, c0, c1);

        if clipped_count < 2 {
            break;
        }

        contact_points = clipped_points.clone();
    }

    let v_ref = &reference_shape.world_vertices()[reference_edge_index];

    let mut result = Vec::new();
    for v_clip in clipped_points {
        let separation = v_clip
            .to_subtracted(v_ref)
            .dot_product(&reference_edge_perpendicular);

        if separation <= 0.0 {
            result.push(Contact::for_polygons(
                a,
                b,
                &ab_separation_info,
                &ba_separation_info,
                &reference_edge_perpendicular,
                &v_clip,
                separation,
            ));
        }
    }

    Some(result)
}

fn are_colliding_circle_and_polygon<'a>(
    circular: &'a RigidBody,
    polygonal: &'a RigidBody,
) -> Option<Vec<Contact<'a>>> {
    let circle = circular
        .shape()
        .circle()
        .unwrap_or_else(|| panic_checked_circle_unwrap());
    let polygon = polygonal
        .shape()
        .polygon()
        .unwrap_or_else(|| panic_checked_polygon_unwrap());

    let mut is_outside = false;
    let mut min_current_vertex = &Vector2::ZERO;
    let mut min_next_vertex = &Vector2::ZERO;
    let mut distance_circle_edge = f64::MIN;

    for (i, current_vertex) in polygon.world_vertices().iter().enumerate() {
        let next_vertex = &polygon.world_vertices()[(i + 1) % polygon.world_vertices().len()];

        let edge = polygon.edge_at(i);
        let normal = edge.create_perpendicular();

        let vertex_to_circle_center = circular.position().to_subtracted(current_vertex);

        let projection = vertex_to_circle_center.dot_product(&normal);

        if projection > 0.0 {
            distance_circle_edge = projection;
            min_current_vertex = current_vertex;
            min_next_vertex = next_vertex;
            is_outside = true;

            break;
        }

        if projection > distance_circle_edge {
            distance_circle_edge = projection;
            min_current_vertex = current_vertex;
            min_next_vertex = next_vertex;
        }
    }

    if is_outside {
        let v1 = circular.position().to_subtracted(min_current_vertex);
        let v2 = min_next_vertex.to_subtracted(min_current_vertex);

        if v1.dot_product(&v2) < 0.0 {
            let v1_magnitude = v1.magnitude();
            if v1_magnitude > circle.radius() {
                return None;
            }

            return Some(vec![Contact::for_circle_and_polygon(
                circular,
                polygonal,
                &v1,
                v1_magnitude,
                false,
            )]);
        }

        let v1 = circular.position().to_subtracted(min_next_vertex);
        let v2 = min_current_vertex.to_subtracted(min_next_vertex);

        let v1_magnitude = v1.magnitude();

        if v1.dot_product(&v2) < 0.0 {
            if v1_magnitude > circle.radius() {
                return None;
            }

            return Some(vec![Contact::for_circle_and_polygon(
                circular,
                polygonal,
                &v1,
                v1_magnitude,
                false,
            )]);
        }

        if distance_circle_edge > circle.radius() {
            return None;
        }

        return Some(vec![Contact::for_circle_and_polygon(
            circular,
            polygonal,
            &min_next_vertex
                .to_subtracted(min_current_vertex)
                .create_perpendicular(),
            distance_circle_edge,
            true,
        )]);
    }

    Some(vec![Contact::for_circle_and_polygon(
        circular,
        polygonal,
        &min_next_vertex
            .to_subtracted(min_current_vertex)
            .create_perpendicular(),
        distance_circle_edge,
        true,
    )])
}

// TODO: Make this SeparationInfo member/constructor
fn find_minimum_separation(a: &Polygon, b: &Polygon) -> SeparationInfo {
    let mut separation = f64::MIN;
    let mut reference_edge_index = 0;

    for (i, va) in a.world_vertices().iter().enumerate() {
        let edge = a.edge_at(i);

        let normal = edge.create_perpendicular();

        let mut min_separation = f64::MAX;

        for vb in b.world_vertices() {
            let projection = vb.to_subtracted(va).dot_product(&normal);

            if projection < min_separation {
                min_separation = projection;
            };
        }

        if min_separation > separation {
            separation = min_separation;
            reference_edge_index = i;
        }
    }

    SeparationInfo {
        separation,
        reference_edge_index,
    }
}
