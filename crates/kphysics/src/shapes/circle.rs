use super::{Shape, ShapeKind};

#[derive(Debug)]
pub struct Circle {
    pub radius: f64,
}

impl Circle {
    pub fn new(radius: f64) -> Self {
        Self { radius }
    }
}

impl Shape for Circle {
    fn get_type(&self) -> ShapeKind {
        ShapeKind::Circle
    }

    fn moment_of_inertia(&self) -> f64 {
        todo!()
    }
}
