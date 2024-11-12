use kmath::Vector2;

#[derive(Debug)]
pub struct Polygon {
    pub world_vertices: Vec<Vector2>,
    local_vertices: Vec<Vector2>,
}

impl Polygon {
    #[inline]
    pub fn new(vertices: Vec<Vector2>) -> Self {
        Self {
            // TODO: Expecting local vertices as an argument may be a foot gun
            world_vertices: vertices.clone(),
            local_vertices: vertices,
        }
    }

    #[inline]
    pub fn rectangular(width: f64, height: f64) -> Self {
        let local_vertices = make_rectangle_vertices(width, height).to_vec();

        Self {
            world_vertices: local_vertices.clone(),
            local_vertices,
        }
    }

    // TODO: store this value
    #[inline]
    pub fn moment_of_inertia(&self) -> f64 {
        todo!()
    }

    #[inline]
    pub fn edge_at(&self, vertex_id: usize) -> Vector2 {
        let next_vertex_id = vertex_id + 1 % self.world_vertices.len();

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
