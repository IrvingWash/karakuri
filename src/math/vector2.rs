#[derive(Debug, Clone)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Default for Vector2 {
    fn default() -> Vector2 {
        Vector2::zero()
    }
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self::new(0., 0.)
    }

    pub fn add(&mut self, other: &Vector2) {
        self.x += other.x;
        self.y += other.y;
    }

    pub fn subtract(&mut self, other: &Vector2) {
        self.x -= other.x;
        self.y -= other.y;
    }

    pub fn scale(&mut self, scaler: f64) {
        self.x *= scaler;
        self.y *= scaler;
    }

    pub fn divide(&mut self, divider: f64) {
        self.x /= divider;
        self.y /= divider;
    }

    pub fn translate(&mut self, increment: f64) {
        self.x += increment;
        self.y += increment;
    }

    pub fn set(&mut self, other: &Vector2) {
        self.x = other.x;
        self.y = other.y;
    }

    pub fn reset(&mut self) {
        self.x = 0.;
        self.y = 0.;
    }

    pub fn squared_magnitude(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2)
    }

    pub fn magnitude(&self) -> f64 {
        self.squared_magnitude().sqrt()
    }

    pub fn dot_product(&self, other: &Vector2) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn cross_product(&self, other: &Vector2) -> f64 {
        self.x * other.y - self.y * other.x
    }

    pub fn normalize(&mut self) {
        let magnitude = self.magnitude();

        self.divide(magnitude);
    }

    pub fn create_perpendicular(&self) -> Vector2 {
        let mut flipped_vector = Vector2::new(self.y, -self.x);

        flipped_vector.normalize();

        flipped_vector
    }

    pub fn rotate(&mut self, angle: f64) {
        let cos = angle.cos();
        let sin = angle.sin();

        let x = self.x * cos - self.y * sin;
        let y = self.x * sin + self.y * cos;

        self.x = x;
        self.y = y;
    }

    pub fn create_copy(&self) -> Vector2 {
        Vector2::new(self.x, self.y)
    }

    pub fn rotate_at(&mut self, pivot: &Vector2, angle: f64) {
        let x = self.x - pivot.x;
        let y = self.y - pivot.y;

        let mut temporary_vector = Vector2::new(x, y);

        temporary_vector.rotate(angle);
        temporary_vector.add(pivot);

        self.set(&temporary_vector);
    }

    pub fn to_added(&self, other: &Vector2) -> Vector2 {
        let mut copy = self.create_copy();

        copy.add(other);

        copy
    }

    pub fn to_subtracted(&self, other: &Vector2) -> Vector2 {
        let mut copy = self.create_copy();

        copy.subtract(other);

        copy
    }

    pub fn to_scaled(&self, scaler: f64) -> Vector2 {
        let mut copy = self.create_copy();

        copy.scale(scaler);

        copy
    }

    pub fn to_divided(&self, divider: f64) -> Vector2 {
        let mut copy = self.create_copy();

        copy.divide(divider);

        copy
    }

    pub fn to_translated(&self, increment: f64) -> Vector2 {
        let mut copy = self.create_copy();

        copy.translate(increment);

        copy
    }

    pub fn to_normalized(&self) -> Vector2 {
        let mut copy = self.create_copy();

        copy.normalize();

        copy
    }

    pub fn to_rotated(&self, angle: f64) -> Vector2 {
        let mut copy = self.create_copy();

        copy.rotate(angle);

        copy
    }

    pub fn to_rotated_at(&self, pivot: &Vector2, angle: f64) -> Vector2 {
        let mut copy = self.create_copy();

        copy.rotate_at(pivot, angle);

        copy
    }
}
