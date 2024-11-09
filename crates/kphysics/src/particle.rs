use kmath::Vector2;

#[derive(Debug, Default)]
pub struct Particle {
    pub position: Vector2,
    pub velocity: Vector2,
    pub mass: f64,
}

impl Particle {
    #[inline]
    pub fn new(position: Vector2, velocity: Vector2, mass: f64) -> Self {
        Self {
            position,
            velocity,
            mass,
        }
    }
}
