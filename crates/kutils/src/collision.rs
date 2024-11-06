use kmath::Vector2;

#[inline]
pub fn aabb_centered(
    position_a: &Vector2,
    size_a: &Vector2,
    position_b: &Vector2,
    size_b: &Vector2,
) -> bool {
    position_a.x < position_b.x + size_b.x
        && position_a.x + size_a.x > position_b.x
        && position_a.y < position_b.y + size_b.y
        && position_a.y + size_a.y > position_b.y
}
