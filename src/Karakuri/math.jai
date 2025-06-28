#import "Math";

deg_to_rad :: (theta: float) -> float {
    return theta * DEG_TO_RAD_CONSTANT;
}

check_aabb :: (
    a_position: Vector2,
    b_position: Vector2,
    a_size: Vector2,
    b_size: Vector2,
    a_origin: Vector2,
    b_origin: Vector2
) -> bool {
    a_x0, a_y0, a_x1, a_y1 := get_aabb(a_position, a_size, a_origin);
    b_x0, b_y0, b_x1, b_y1 := get_aabb(b_position, b_size, b_origin);

    return !(
        a_x1 < b_x0
        || a_x0 > b_x1
        || a_y1 > b_y0
        || a_y0 < b_y1
    );
}

#scope_file

DEG_TO_RAD_CONSTANT :: PI / 180;

get_aabb :: (
    position: Vector2,
    size: Vector2,
    origin: Vector2
) -> x0: float, y0: float, x1: float, y1: float {
    x0 := position.x - size.x * origin.x;
    y0 := position.y + size.y * origin.y;
    x1 := x0 + size.x;
    y1 := y0 - size.y;

    return x0, y0, x1, y1;
}
