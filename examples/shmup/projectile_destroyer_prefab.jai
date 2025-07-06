projectile_destroyer_prefab :: (position: enum {Top; Bottom;}) -> Karakuri.Entity_Params {
    return .{
        transform = .{
            position = .{
                0,
                ifx position == .Top {
                    8.0;
                } else {
                    -8.0;
                }
            },
        },
        box_collider_component = Karakuri.Maybe.some(
            Karakuri.Box_Collider_Component.{
                size = .{100, 0.1},
            }
        ),

        on_collision = on_collision,
    };
}

#scope_file

on_collision :: (self: *Karakuri.Entity, other: *Karakuri.Entity) {
    if Karakuri.entity_has_tags(other, "projectile") {
        Karakuri.despawn_entity(other);
    }
}
