#scope_module
#import "Basic";

physics_system :: () {
    entities := world_find_entities_with_components(Box_Collider_Component, Sprite_Component);

    for a, i: entities {
        j := i + 1;

        ok, a_sprite := world_get_component(a, Sprite_Component);
        assert(ok);

        for j..entities.count - 1 {
            b := entities[j];
            ok =, b_sprite := world_get_component(b, Sprite_Component);

            if check_aabb(
                a.transform.position,
                b.transform.position,
                a.transform.scale,
                b.transform.scale,
                a_sprite.origin,
                b_sprite.origin,
            ) {
                print("Colliding!");
            }
        }
    }
}

#scope_file

check_aabb :: (
    a_position: Vector2,
    b_position: Vector2,
    a_size: Vector2,
    b_size: Vector2,
    a_origin: Vector2,
    b_origin: Vector2
) -> bool {
    // @Todo why * 2? Dpi? Can't confirm as this works both on MacBook Air and on my Linux machine
    // Try to implement pixels per unit to check this out
    a_x0, a_y0, a_x1, a_y1 := get_aabb(a_position, a_size * 2, a_origin);
    b_x0, b_y0, b_x1, b_y1 := get_aabb(b_position, b_size * 2, b_origin);

    return !(
        a_x1 < b_x0
        || a_x0 > b_x1
        || a_y1 > b_y0
        || a_y0 < b_y1
    );
}

get_aabb :: (
    position: Vector2,
    size: Vector2,
    origin: Vector2
) -> x0: float, y0: float, x1: float, y1: float {
    x0 := position.x - size.x * 0.5 * (1 - origin.x);
    y0 := position.y + size.y * 0.5 * (1 + origin.y);
    x1 := x0 + size.x;
    y1 := y0 - size.y;

    return x0, y0, x1, y1;
}
