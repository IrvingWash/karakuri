const I: f64 = 1.0 / 2.0;

#[derive(Debug)]
pub struct Circle {
    pub radius: f64,
    moment_of_inertia: f64,
}

impl Circle {
    #[inline]
    pub fn new(radius: f64) -> Self {
        Self {
            radius,
            moment_of_inertia: I * radius.powi(2),
        }
    }

    #[inline]
    pub fn moment_of_inertia(&self) -> f64 {
        self.moment_of_inertia
    }
}
