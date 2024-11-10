use kmath::Vector2;

#[derive(Debug)]
pub struct Polygon {
    pub vertices: Vec<Vector2>,
}

impl Polygon {
    pub fn new(vertices: Vec<Vector2>) -> Self {
        Self { vertices }
    }

    pub const MOMENT_OF_INERTIA: f64 = 0.0;
}
