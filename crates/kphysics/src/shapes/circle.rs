#[derive(Debug)]
pub struct Circle {
    pub radius: f64,
}

impl Circle {
    #[inline]
    pub fn new(radius: f64) -> Self {
        Self { radius }
    }

    // TODO: Store this value
    #[inline]
    pub fn moment_of_inertia(&self) -> f64 {
        0.5 * (self.radius.powi(2))
    }
}
