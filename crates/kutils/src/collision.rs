use kmath::Vector2;

#[inline]
pub fn aabb(
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

#[cfg(test)]
mod aabb_tests {
    use kmath::Vector2;

    use crate::collision::aabb;

    #[test]
    fn test_aabb() {
        let rect_a_position = Vector2::new(0.0, 0.0);
        let rect_a_size = Vector2::new(10.0, 10.0);

        let rect_b_position = Vector2::new(9.0, 9.0);
        let rect_b_size = Vector2::new(10.0, 10.0);

        assert!(aabb(
            &rect_a_position,
            &rect_a_size,
            &rect_b_position,
            &rect_b_size
        ));

        let rect_b_position = Vector2::new(10.0, 10.0);
        let rect_b_size = Vector2::new(11.0, 11.0);

        assert!(!aabb(
            &rect_a_position,
            &rect_a_size,
            &rect_b_position,
            &rect_b_size
        ));
    }
}
