use super::{Shape, ShapeKind};

#[derive(Debug)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }
}

impl Shape for Rectangle {
    fn get_type(&self) -> ShapeKind {
        ShapeKind::Rectangle
    }

    fn moment_of_inertia(&self) -> f64 {
        todo!()
    }
}
