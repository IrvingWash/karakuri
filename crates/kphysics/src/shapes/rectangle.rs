use kmath::Vector2;

const MOMENT_OF_INERTIA_K: f64 = 1.0 / 12.0;

// TODO: Do we really need this struct? Isn't `Polygon` enough?
#[derive(Debug)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
    pub world_vertices: [Vector2; 4],
    local_vertices: [Vector2; 4],
}

impl Rectangle {
    #[inline]
    pub fn new(width: f64, height: f64) -> Self {
        let vertices = make_vertices(width, height);

        Self {
            width,
            height,
            world_vertices: vertices.clone(),
            local_vertices: vertices,
        }
    }

    #[inline]
    pub fn moment_of_inertia(&self) -> f64 {
        MOMENT_OF_INERTIA_K * (self.width.powi(2) + self.height.powi(2))
    }

    #[inline]
    pub fn update_vertices(&mut self, position: &Vector2, rotation: f64) {
        // TODO: DRY! (check rectangle)
        for (i, vertex) in self.world_vertices.iter_mut().enumerate() {
            vertex.set(&self.local_vertices[i].to_rotated(rotation));
            vertex.add(position);
        }
    }
}

fn make_vertices(width: f64, height: f64) -> [Vector2; 4] {
    let half_width = width / 2.;
    let half_height = height / 2.;

    [
        Vector2::new(-half_width, -half_height),
        Vector2::new(half_width, -half_height),
        Vector2::new(half_width, half_height),
        Vector2::new(-half_width, half_height),
    ]
}
