enemy_prefab :: (position: Vector2) -> Karakuri.Entity_Params {
    tags: [..]string;
    array_add(
        *tags,
        "enemy"
    );

    return .{
        tags = tags,
        transform = .{
            position = position,
            scale = .{1, 1},
        },
        sprite_component = Karakuri.Maybe.some(Karakuri.Sprite_Component.{
            texture_path = "examples/shmup/assets/enemy_2_1.png",
        }),
        box_collider_component = Karakuri.Maybe.some(
            Karakuri.Box_Collider_Component.{}
        ),
        on_collision = on_collision,
    };
}

#scope_file

on_collision :: (self: *Karakuri.Entity, other: *Karakuri.Entity) {
    if other.tags.count > 0 && other.tags[0] == "projectile" {
        Karakuri.despawn_entity(self);
    }
}
