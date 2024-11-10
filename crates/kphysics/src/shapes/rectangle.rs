use kmath::Vector2;

const MOMENT_OF_INERTIA_K: f64 = 1.0 / 12.0;

#[derive(Debug)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
    pub vertices: [Vector2; 4],
}

impl Rectangle {
    #[inline]
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            width,
            height,
            vertices: make_vertices(width, height),
        }
    }

    #[inline]
    pub fn moment_of_inertia(&self) -> f64 {
        MOMENT_OF_INERTIA_K * (self.width.powi(2) + self.height.powi(2))
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
