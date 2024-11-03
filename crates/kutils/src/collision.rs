use kmath::Vector2;

pub fn aabb_centered(
    position_a: &Vector2,
    size_a: &Vector2,
    position_b: &Vector2,
    size_b: &Vector2,
) -> bool {
    let half_width_a = size_a.x * 0.5;
    let half_width_b = size_b.x * 0.5;

    let half_height_a = size_a.y * 0.5;
    let half_height_b = size_b.y * 0.5;

    position_a.x < position_b.x + half_width_b
        && position_a.x + half_width_a > position_b.x
        && position_a.y - half_height_a < position_b.y + half_height_b
        && position_a.y + half_height_a > position_b.y - half_height_b
}
