use kmath::Vector2;

use super::{Shape, ShapeKind};

#[derive(Debug)]
pub struct Polygon {
    pub vertices: Vec<Vector2>,
}

impl Polygon {
    pub fn new(vertices: Vec<Vector2>) -> Self {
        Self { vertices }
    }
}

impl Shape for Polygon {
    fn get_type(&self) -> ShapeKind {
        ShapeKind::Polygon
    }

    fn moment_of_inertia(&self) -> f64 {
        todo!()
    }
}
