use kmath::Vector2;

use crate::Particle;

pub fn weight(particle: &mut Particle, k: f64) {
    particle.apply_force(&Vector2::new(0.0, 9.8 * k * particle.mass));
}
