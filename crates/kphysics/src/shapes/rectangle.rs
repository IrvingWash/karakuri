const MOMENT_OF_INERTIA_K: f64 = 1.0 / 12.0;

#[derive(Debug)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    #[inline]
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }

    #[inline]
    pub fn moment_of_inertia(&self) -> f64 {
        MOMENT_OF_INERTIA_K * (self.width.powi(2) + self.height.powi(2))
    }
}
