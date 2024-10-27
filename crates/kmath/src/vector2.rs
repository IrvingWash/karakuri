#[derive(Debug, Clone, PartialEq)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Default for Vector2 {
    fn default() -> Vector2 {
        Vector2::ZERO
    }
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Vector2 {
        Self { x, y }
    }

    pub const ZERO: Vector2 = Vector2 { x: 0., y: 0. };

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
        if divider == 0. {
            return;
        }

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

    pub fn move_towards(&mut self, other: &Vector2, max_distance: f64) {
        let mut disposition = other.to_subtracted(self);

        let magnitude = disposition.magnitude();

        if magnitude <= max_distance || magnitude == 0.0 {
            self.set(other);
        }

        disposition.divide(magnitude);
        disposition.scale(max_distance);
        disposition.add(self);

        self.set(&disposition);
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

    pub fn to_moved_towards(&self, other: &Vector2, max_distance: f64) -> Vector2 {
        let mut copy = self.create_copy();

        copy.move_towards(other, max_distance);

        copy
    }
}

#[cfg(test)]
mod vector2_tests {
    use super::Vector2;

    const NINETY_DEGREES_IN_RADIANS: f64 = 1.5708;

    const X1: f64 = 2.;
    const Y1: f64 = -34.4;
    const X2: f64 = 432.;
    const Y2: f64 = 0.;

    #[test]
    fn test_new() {
        let v = Vector2::new(X1, Y1);

        assert_eq!(v.x, X1);
        assert_eq!(v.y, Y1);
    }

    #[test]
    fn test_zero() {
        let v = Vector2::ZERO;

        assert_eq!(v.x, 0.);
        assert_eq!(v.y, 0.);
    }

    #[test]
    fn test_add() {
        let mut v = Vector2::new(X1, Y1);
        let other = Vector2::new(X2, Y2);

        v.add(&other);

        assert_eq!(v.x, X1 + X2);
        assert_eq!(v.y, Y1 + Y2);
    }

    #[test]
    fn test_to_added() {
        let v = Vector2::new(X1, Y1);
        let other = Vector2::new(X2, Y2);

        let result = v.to_added(&other);

        assert_eq!(result.x, X1 + X2);
        assert_eq!(result.y, Y1 + Y2);
    }

    #[test]
    fn test_subtract() {
        let mut v = Vector2::new(X1, Y1);
        let other = Vector2::new(X2, Y2);

        v.subtract(&other);

        assert_eq!(v.x, X1 - X2);
        assert_eq!(v.y, Y1 - Y2);
    }

    #[test]
    fn test_to_subtracted() {
        let v = Vector2::new(X1, Y1);
        let other = Vector2::new(X2, Y2);

        let result = v.to_subtracted(&other);

        assert_eq!(result.x, X1 - X2);
        assert_eq!(result.y, Y1 - Y2);
    }

    #[test]
    fn test_scale() {
        let mut v = Vector2::new(X1, Y1);

        v.scale(X2);

        assert_eq!(v.x, X1 * X2);
        assert_eq!(v.y, Y1 * X2);
    }

    #[test]
    fn test_to_scaled() {
        let v = Vector2::new(X1, Y1);

        let result = v.to_scaled(X2);

        assert_eq!(result.x, X1 * X2);
        assert_eq!(result.y, Y1 * X2);
    }

    #[test]
    fn test_divide() {
        let mut v = Vector2::new(X1, Y1);

        v.divide(X2);

        assert_eq!(v.x, X1 / X2);
        assert_eq!(v.y, Y1 / X2);
    }

    #[test]
    fn test_translate() {
        let mut v = Vector2::new(X1, Y1);
        let increment = 3.;

        v.translate(increment);

        assert_eq!(v.x, X1 + increment);
        assert_eq!(v.y, Y1 + increment);
    }

    #[test]
    fn test_to_divided() {
        let v = Vector2::new(X1, Y1);

        let result = v.to_divided(X2);

        assert_eq!(result.x, X1 / X2);
        assert_eq!(result.y, Y1 / X2);
    }

    #[test]
    fn test_to_translated() {
        let v = Vector2::new(X1, Y1);
        let increment = 3.;

        let result = v.to_translated(increment);

        assert_eq!(result.x, X1 + increment);
        assert_eq!(result.y, Y1 + increment);
    }

    #[test]
    fn test_set() {
        let mut v = Vector2::new(X1, Y1);

        v.set(&Vector2::new(X2, Y2));

        assert_eq!(v.x, X2);
        assert_eq!(v.y, Y2);
    }

    #[test]
    fn test_reset() {
        let mut v = Vector2::new(X1, Y1);

        v.reset();

        assert_eq!(v.x, 0.);
        assert_eq!(v.y, 0.);
    }

    #[test]
    fn test_squared_magnitude() {
        let v = Vector2::new(X1, Y1);

        assert_eq!(v.squared_magnitude(), 1187.36);
    }

    #[test]
    fn test_magnitude() {
        let v = Vector2::new(X1, Y1);

        assert_eq!(v.magnitude(), 34.458090486850836);
    }

    #[test]
    fn test_dot_product() {
        let v = Vector2::new(X2, Y2);
        let other = Vector2::new(-32., -99.);

        let dot_product = v.dot_product(&other);

        assert_eq!(dot_product, -13824.);
    }

    #[test]
    fn test_cross_product() {
        let v = Vector2::new(432.0, 0.);
        let other = Vector2::new(-32., -99.);

        let cross_product = v.cross_product(&other);

        assert_eq!(cross_product, -42768.);
    }

    #[test]
    fn test_normalize() {
        let mut v = Vector2::new(-234., 309.);

        v.normalize();

        assert_eq!(v.x, -0.6037086604052452);
        assert_eq!(v.y, 0.7972050259197468);
    }

    #[test]
    fn test_to_normalized() {
        let v = Vector2::new(-234., 309.);

        let result = v.to_normalized();

        assert_eq!(result.x, -0.6037086604052452);
        assert_eq!(result.y, 0.7972050259197468);
    }

    #[test]
    fn test_create_perpendicular() {
        let v = Vector2::new(-102.23, 34.);

        let perpendicular = v.create_perpendicular();

        assert_eq!(perpendicular.x, 0.3155872375021863);
        assert_eq!(perpendicular.y, 0.9488965673484855);
    }

    #[test]
    fn test_rotate() {
        let mut v = Vector2::new(34.343, -27.);

        v.rotate(NINETY_DEGREES_IN_RADIANS);

        assert_eq!(v.x, 26.99987385093499);
        assert_eq!(v.y, 34.343099176306104);
    }

    #[test]
    fn test_to_rotated() {
        let v = Vector2::new(34.343, -27.);

        let result = v.to_rotated(NINETY_DEGREES_IN_RADIANS);

        assert_eq!(result.x, 26.99987385093499);
        assert_eq!(result.y, 34.343099176306104);
    }

    #[test]
    fn test_rotate_at() {
        let mut v = Vector2::new(34., -27.);
        let pivot = Vector2::new(34. * 0.5, -27. * 0.5);

        v.rotate_at(&pivot, NINETY_DEGREES_IN_RADIANS);

        assert_eq!(v.x, 30.49993755542217);
        assert_eq!(v.y, 3.5000495881542086);
    }

    #[test]
    fn test_to_rotated_at() {
        let v = Vector2::new(34., -27.);
        let pivot = Vector2::new(34. * 0.5, -27. * 0.5);

        let result = v.to_rotated_at(&pivot, NINETY_DEGREES_IN_RADIANS);

        assert_eq!(result.x, 30.49993755542217);
        assert_eq!(result.y, 3.5000495881542086);
    }

    #[test]
    fn test_create_copy() {
        let v = Vector2::new(X1, Y1);

        let copy = v.create_copy();

        assert_eq!(copy.x, X1);
        assert_eq!(copy.y, Y1);
    }
}
