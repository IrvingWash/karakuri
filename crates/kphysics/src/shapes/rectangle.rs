#[derive(Debug)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }

    pub const MOMENT_OF_INERTIA: f64 = 0.0;
}
