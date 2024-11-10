use kmath::Vector2;

#[derive(Debug)]
pub struct Polygon {
    pub vertices: Vec<Vector2>,
}

impl Polygon {
    #[inline]
    pub fn new(vertices: Vec<Vector2>) -> Self {
        Self { vertices }
    }

    #[inline]
    pub fn moment_of_inertia(&self) -> f64 {
        todo!()
    }
}
