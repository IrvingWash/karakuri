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
            // TODO: Expecting local vertices here may be a foot gun
            world_vertices: vertices.clone(),
            local_vertices: vertices,
        }
    }

    #[inline]
    pub fn moment_of_inertia(&self) -> f64 {
        todo!()
    }

    #[inline]
    pub fn update_vertices(&mut self, position: &Vector2, rotation: f64) {
        for (i, vertex) in self.world_vertices.iter_mut().enumerate() {
            vertex.set(&self.local_vertices[i].to_rotated(rotation));
            vertex.add(position);
        }
    }
}
