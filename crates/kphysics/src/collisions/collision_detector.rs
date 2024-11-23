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
    let a_polygon = a
        .shape
        .polygon()
        .unwrap_or_else(|| panic_checked_polygon_unwrap());
    let b_polygon = b
        .shape
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

    Some(Contact::for_polygons(
        a,
        b,
        ab_separation_info,
        ba_separation_info,
    ))
}

fn are_colliding_circle_and_polygon<'a>(
    circular: &'a mut RigidBody,
    polygonal: &'a mut RigidBody,
) -> Option<Contact<'a>> {
    let circle = circular
        .shape
        .circle()
        .unwrap_or_else(|| panic_checked_circle_unwrap());
    let polygon = polygonal
        .shape
        .polygon()
        .unwrap_or_else(|| panic_checked_polygon_unwrap());

    let mut is_outside = false;
    let mut min_current_vertex = &Vector2::ZERO;
    let mut min_next_vertex = &Vector2::ZERO;
    let mut distance_circle_edge = f64::MIN;

    for (i, current_vertex) in polygon.world_vertices.iter().enumerate() {
        let next_vertex = &polygon.world_vertices[(i + 1) % polygon.world_vertices.len()];

        let edge = polygon.edge_at(i);
        let normal = edge.create_perpendicular();

        let vertex_to_circle_center = circular.position.to_subtracted(current_vertex);

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
        let v1 = circular.position.to_subtracted(min_current_vertex);
        let v2 = min_next_vertex.to_subtracted(min_current_vertex);

        if v1.dot_product(&v2) < 0.0 {
            let v1_magnitude = v1.magnitude();
            if v1_magnitude > circle.radius {
                return None;
            }

            return Some(Contact::for_circle_and_polygon(
                circular,
                polygonal,
                &v1,
                v1_magnitude,
                false,
            ));
        }

        let v1 = circular.position.to_subtracted(min_next_vertex);
        let v2 = min_current_vertex.to_subtracted(min_next_vertex);

        let v1_magnitude = v1.magnitude();

        if v1.dot_product(&v2) < 0.0 {
            if v1_magnitude > circle.radius {
                return None;
            }

            return Some(Contact::for_circle_and_polygon(
                circular,
                polygonal,
                &v1,
                v1_magnitude,
                false,
            ));
        }

        if distance_circle_edge > circle.radius {
            return None;
        }

        return Some(Contact::for_circle_and_polygon(
            circular,
            polygonal,
            &min_next_vertex
                .to_subtracted(min_current_vertex)
                .create_perpendicular(),
            distance_circle_edge,
            true,
        ));
    }

    Some(Contact::for_circle_and_polygon(
        circular,
        polygonal,
        &min_next_vertex
            .to_subtracted(min_current_vertex)
            .create_perpendicular(),
        distance_circle_edge,
        true,
    ))
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

#[cfg(test)]
mod collision_detector_tests {
    use kmath::Vector2;

    use crate::{shapes::Shape, RigidBody, RigidBodyParams};

    use super::are_colliding;

    #[test]
    fn test_circles() {
        let mut a = RigidBody::new(RigidBodyParams {
            position: Vector2::new(0.0, 0.0),
            shape: Shape::new_circle(1.0),
            ..Default::default()
        });
        let mut b = RigidBody::new(RigidBodyParams {
            position: Vector2::new(10.0, 10.0),
            shape: Shape::new_circle(1.0),
            ..Default::default()
        });

        assert!(are_colliding(&mut a, &mut b).is_none());

        a.position = Vector2::new(10.0, 10.0);

        let contact = are_colliding(&mut a, &mut b).unwrap();

        assert_eq!(contact.end, Vector2 { x: 10.0, y: 10.0 });
        assert_eq!(contact.normal, Vector2::ZERO);
        assert_eq!(contact.start, Vector2 { x: 10.0, y: 10.0 });
    }

    #[test]
    fn test_polygons() {
        let mut a = RigidBody::new(RigidBodyParams {
            position: Vector2::new(0.0, 0.0),
            shape: Shape::new_rectangle(5.0, 5.0),
            ..Default::default()
        });
        let mut b = RigidBody::new(RigidBodyParams {
            position: Vector2::new(10.0, 10.0),
            shape: Shape::new_rectangle(10.0, 10.0),
            ..Default::default()
        });

        assert!(are_colliding(&mut a, &mut b).is_none());

        a.position = Vector2::new(9.0, 9.0);

        a.update(1.0);
        b.update(1.0);

        let contact = are_colliding(&mut a, &mut b).unwrap();

        assert_eq!(contact.end, Vector2 { x: 11.5, y: 11.5 });
        assert_eq!(contact.normal, Vector2 { x: -0.0, y: 1.0 });
        assert_eq!(contact.start, Vector2 { x: 11.5, y: 5.0 });
    }
}
