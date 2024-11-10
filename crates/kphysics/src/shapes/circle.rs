#[derive(Debug)]
pub struct Circle {
    pub radius: f64,
}

impl Circle {
    pub fn new(radius: f64) -> Self {
        Self { radius }
    }

    pub const MOMENT_OF_INERTIA: f64 = 0.0;
}
