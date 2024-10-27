use kmath::Vector2;

use crate::Size;

pub fn aabb_centered(
    position_a: &Vector2,
    size_a: &Size,
    position_b: &Vector2,
    size_b: &Size,
) -> bool {
    let half_width_a = size_a.width as f64 * 0.5;
    let half_width_b = size_b.width as f64 * 0.5;

    let half_height_a = size_a.height as f64 * 0.5;
    let half_height_b = size_b.height as f64 * 0.5;

    position_a.x < position_b.x + half_width_b
        && position_a.x + half_width_a > position_b.x
        && position_a.y - half_height_a < position_b.y + half_height_b
        && position_a.y + half_height_a > position_b.y - half_height_b
}
