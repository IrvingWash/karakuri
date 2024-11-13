use kmath::Vector2;

#[derive(Debug)]
pub struct Polygon {
    pub world_vertices: Vec<Vector2>,
    local_vertices: Vec<Vector2>,
    moment_of_inertia: f64,
}

impl Polygon {
    #[inline]
    pub fn new(vertices: Vec<Vector2>) -> Self {
        Self {
            // TODO: Expecting local vertices as an argument may be a foot gun
            world_vertices: vertices.clone(),
            local_vertices: vertices,
            moment_of_inertia: 0.0, // TODO
        }
    }

    #[inline]
    pub fn rectangular(width: f64, height: f64) -> Self {
        let local_vertices = make_rectangle_vertices(width, height).to_vec();

        Self {
            world_vertices: local_vertices.clone(),
            local_vertices,
            moment_of_inertia: 0.0, // TODO
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
