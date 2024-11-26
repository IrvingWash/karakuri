use kmath::Vector2;

const RECTANGLE_MOI_K: f64 = 1.0 / 12.0;

#[derive(Debug)]
pub struct Polygon {
    world_vertices: Vec<Vector2>,
    local_vertices: Vec<Vector2>,
    moment_of_inertia: f64,
}

impl Polygon {
    #[inline]
    pub fn new(vertices: Vec<Vector2>) -> Self {
        Self {
            world_vertices: vertices.clone(),
            local_vertices: vertices,
            moment_of_inertia: 5000.0, // TODO
        }
    }

    #[inline]
    pub fn rectangular(width: f64, height: f64) -> Self {
        let local_vertices = make_rectangle_vertices(width, height).to_vec();

        Self {
            world_vertices: local_vertices.clone(),
            local_vertices,
            moment_of_inertia: RECTANGLE_MOI_K * (width.powi(2) + height.powi(2)),
        }
    }

    #[inline]
    pub fn moment_of_inertia(&self) -> f64 {
        self.moment_of_inertia
    }

    #[inline]
    pub fn edge_at(&self, vertex_id: usize) -> Vector2 {
        let next_vertex_id = (vertex_id + 1) % self.world_vertices.len();

        self.world_vertices[next_vertex_id].to_subtracted(&self.world_vertices[vertex_id])
    }

    #[inline]
    pub fn update_vertices(&mut self, position: &Vector2, rotation: f64) {
        for (i, vertex) in self.world_vertices.iter_mut().enumerate() {
            vertex.set(&self.local_vertices[i].to_rotated(rotation));
            vertex.add(position);
        }
    }

    #[inline]
    pub fn world_vertices(&self) -> &Vec<Vector2> {
        &self.world_vertices
    }

    // TODO: Don't like this method in Polygon struct.
    // Maybe should move to Contact/Collision detector
    #[inline]
    pub fn find_incident_edge_index(&self, reference_normal: &Vector2) -> usize {
        let mut incident_edge_index = 0;
        let mut min_projection = f64::MAX;

        for i in 0..self.world_vertices.len() {
            let edge_normal = self.edge_at(i).create_perpendicular();

            let projection = edge_normal.dot_product(reference_normal);

            if projection < min_projection {
                min_projection = projection;
                incident_edge_index = i;
            }
        }

        incident_edge_index
    }

    // TODO: Remove out parameter (sucks)
    #[inline]
    pub fn clip_segment_to_line(
        &self,
        contacts_in: &[Vector2],
        contacts_out: &mut [Vector2],
        c0: &Vector2,
        c1: &Vector2,
    ) -> usize {
        let mut out_count = 0;

        let normal = c1.to_subtracted(c0).to_normalized();
        let dist0 = contacts_in[0].to_subtracted(c0).cross_product(&normal);
        let dist1 = contacts_in[1].to_subtracted(c0).cross_product(&normal);

        if dist0 <= 0.0 {
            contacts_out[out_count] = contacts_in[0].clone();
            out_count += 1;
        }
        if dist1 <= 0.0 {
            contacts_out[out_count] = contacts_in[1].clone();
            out_count += 1;
        }

        if dist0 * dist1 < 0.0 {
            let total_dist = dist0 - dist1;

            let t = dist0 / total_dist;

            let contact = contacts_in[0]
                .to_added(&contacts_in[1].to_subtracted(&contacts_in[0]).to_scaled(t));

            contacts_out[out_count] = contact;

            out_count += 1;
        }

        out_count
    }
}

fn make_rectangle_vertices(width: f64, height: f64) -> [Vector2; 4] {
    let half_width = width / 2.;
    let half_height = height / 2.;

    [
        Vector2::new(-half_width, -half_height),
        Vector2::new(half_width, -half_height),
        Vector2::new(half_width, half_height),
        Vector2::new(-half_width, half_height),
    ]
}

#[cfg(test)]
mod polygon_tests {
    use kmath::Vector2;

    use super::Polygon;

    #[test]
    fn test_rectangular() {
        let polygon = Polygon::rectangular(30.0, 31.0);

        assert_eq!(
            polygon.local_vertices,
            vec![
                Vector2 { x: -15.0, y: -15.5 },
                Vector2 { x: 15.0, y: -15.5 },
                Vector2 { x: 15.0, y: 15.5 },
                Vector2 { x: -15.0, y: 15.5 }
            ]
        );
    }

    #[test]
    fn test_edge_at() {
        let polygon = Polygon::rectangular(30.0, 31.0);

        assert_eq!(polygon.edge_at(1), Vector2 { x: 0.0, y: 31.0 });
    }
}
