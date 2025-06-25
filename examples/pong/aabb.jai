#import "Math";

check_aabb_centered :: (a: Vector2, b: Vector2, a_size: Vector2, b_size: Vector2) -> bool {
    a_half_width := a_size.x / 2;
    a_half_height := a_size.y / 2;
    b_half_width := b_size.x / 2;
    b_half_height := b_size.y / 2;

    return (
        abs(a.x - b.x) < a_half_width + b_half_width &&
        abs(a.y - b.y) < a_half_height + b_half_height
    );
}
