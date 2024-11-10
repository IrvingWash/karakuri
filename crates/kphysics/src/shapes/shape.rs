pub enum ShapeKind {
    Circle,
    Polygon,
    Rectangle,
}

pub trait Shape {
    fn get_type(&self) -> ShapeKind;
    fn moment_of_inertia(&self) -> f64;
}
