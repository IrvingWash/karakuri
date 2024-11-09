use kmath::Vector2;

pub fn weight(mass: f64, k: f64) -> Vector2 {
    Vector2::new(0.0, 9.8 * k * mass)
}
