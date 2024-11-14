use kmath::Vector2;

#[derive(Debug)]
pub struct SeparationInfo {
    pub separation: f64,
    pub separation_axis: Vector2,
    pub point: Vector2,
}
