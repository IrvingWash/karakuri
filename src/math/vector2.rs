use std::{
    cmp::Ordering,
    ops::{
        Add, AddAssign,
        Neg,
        Sub, SubAssign,
        Mul, MulAssign,
        Div, DivAssign,
        Rem, RemAssign,
    },
};

#[derive(Debug, Copy, Clone, PartialEq)]
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
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub const ZERO: Vector2 = Vector2 { x: 0., y: 0. };

    pub fn dot(self, other: Vector2) -> f64 {
        let t = self * other;
        t.x + t.y
    }

    pub fn squared_magnitude(self) -> f64 {
        self.dot(self)
    }

    pub fn magnitude(self) -> f64 {
        self.squared_magnitude().sqrt()
    }

    pub fn cross(self, other: Vector2) -> f64 {
        self.x * other.y - self.y * other.x
    }

    pub fn normalize(&mut self) {
        let magnitude = self.magnitude();

        *self /= magnitude;
    }

    pub fn create_perpendicular(self) -> Self {
        let mut flipped_vector = Vector2::new(self.y, -self.x);

        flipped_vector.normalize();

        flipped_vector
    }

    pub fn rotate(&mut self, angle: f64) {
        let cos = angle.cos();
        let sin = angle.sin();

        let x = self.x * cos - self.y * sin;
        let y = self.x * sin + self.y * cos;

        *self = (x, y).into();
    }

    pub fn rotate_at(&mut self, pivot: Vector2, angle: f64) {
        let mut t = *self - pivot;

        t.rotate(angle);
        t += pivot;

        *self = t;
    }

    pub fn clamp(self, min: Vector2, max: Vector2) -> Self {
        Self {
            x: self.x.clamp(min.x, max.x),
            y: self.y.clamp(min.y, max.y),
        }
    }

    pub fn rem_euclid(self, other: Vector2) -> Self {
        Self {
            x: self.x.rem_euclid(other.x),
            y: self.y.rem_euclid(other.y),
        }
    }
}

impl From<f64> for Vector2 {
    fn from(s: f64) -> Self {
        Self {
            x: s,
            y: s,
        }
    }
}

impl From<(f64, f64)> for Vector2 {
    fn from((x, y): (f64, f64)) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl From<[f64; 2]> for Vector2 {
    fn from([x, y]: [f64; 2]) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, other: Vector2) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<f64> for Vector2 {
    type Output = Self;

    fn add(self, other: f64) -> Self {
        let other: Vector2 = other.into();

        self + other
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, other: Vector2) {
        *self = *self + other;
    }
}

impl AddAssign<f64> for Vector2 {
    fn add_assign(&mut self, other: f64) {
        let other: Vector2 = other.into();

        *self *= other;
    }
}

impl Neg for Vector2 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, other: Vector2) -> Self {
        self + -other
    }
}

impl Sub<f64> for Vector2 {
    type Output = Self;

    fn sub(self, other: f64) -> Self {
        let other: Vector2 = other.into();

        self - other
    }
}

impl SubAssign for Vector2 {
    fn sub_assign(&mut self, other: Vector2) {
        *self = *self - other;
    }
}

impl SubAssign<f64> for Vector2 {
    fn sub_assign(&mut self, other: f64) {
        let other: Vector2 = other.into();

        *self -= other;
    }
}

impl Mul for Vector2 {
    type Output = Self;

    fn mul(self, other: Vector2) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl Mul<f64> for Vector2 {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        let other: Vector2 = other.into();

        self * other
    }
}

impl MulAssign for Vector2 {
    fn mul_assign(&mut self, other: Vector2) {
        *self = *self * other;
    }
}

impl MulAssign<f64> for Vector2 {
    fn mul_assign(&mut self, other: f64) {
        let other: Vector2 = other.into();

        *self *= other;
    }
}

impl Div for Vector2 {
    type Output = Self;

    fn div(self, other: Vector2) -> Vector2 {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl Div<f64> for Vector2 {
    type Output = Self;

    fn div(self, other: f64) -> Vector2 {
        let other: Vector2 = other.into();

        self / other
    }
}

impl DivAssign for Vector2 {
    fn div_assign(&mut self, other: Vector2) {
        *self = *self / other;
    }
}

impl DivAssign<f64> for Vector2 {
    fn div_assign(&mut self, other: f64) {
        let other: Vector2 = other.into();

        *self /= other;
    }
}

impl Rem for Vector2 {
    type Output = Self;

    fn rem(self, other: Vector2) -> Self {
        Self {
            x: self.x % other.x,
            y: self.y % other.y,
        }
    }
}

impl RemAssign for Vector2 {
    fn rem_assign(&mut self, other: Vector2) {
        *self = *self % other;
    }
}

impl PartialOrd for Vector2 {
    fn partial_cmp(&self, other: &Vector2) -> Option<Ordering> {
        let this_sq = self.squared_magnitude();
        let other_sq = other.squared_magnitude();
        this_sq.partial_cmp(&other_sq)
    }
}

impl std::fmt::Display for Vector2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "({},{})", self.x, self.y)
    }
}
