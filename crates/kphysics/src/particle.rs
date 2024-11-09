use kmath::Vector2;

#[derive(Debug, Default)]
pub struct Particle {
    pub position: Vector2,
    pub velocity: Vector2,
    pub mass: f64,
    pub radius: f64, // For debug
}

impl Particle {
    #[inline]
    pub fn new(position: Vector2, velocity: Vector2, mass: f64, radius: f64) -> Self {
        Self {
            position,
            velocity,
            mass,
            radius,
        }
    }
}
