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
        behavior_component = Karakuri.Maybe.some(
            Karakuri.instantiate_behavior(Enemy.{}),
        ),
        sprite_component = Karakuri.Maybe.some(Karakuri.Sprite_Component.{
            texture_path = "examples/shmup/assets/enemy_2_1.png",
        }),
        box_collider_component = Karakuri.Maybe.some(
            Karakuri.Box_Collider_Component.{
                size = .{0.38, 0.34},
            }
        ),

        on_start = on_start,
        on_collision = on_collision,
        on_destroy = on_destroy,
    };
}

Enemy :: struct {
    using #as behavior: Karakuri.Behavior_Component;
    shoot_interval: u32;
}

#scope_file

on_start :: (self: *Karakuri.Entity) {
    ok, enemy := Karakuri.get_behavior(self, Enemy);
    assert(ok);

    // @Todo this sucks. MAX_ENTITIES?
    token := New(Karakuri.Entity_Token);
    token.* = self.token;
    enemy.shoot_interval = Karakuri.set_interval(
        1000,
        (data: *void) {
            ok, self := Karakuri.get_entity_with_token(cast(*Karakuri.Entity_Token) data);
            assert(ok);

            Karakuri.spawn_entity(projectile_prefab(self.transform.position, .Enemy));
        },
        token,
    );
}

on_collision :: (self: *Karakuri.Entity, other: *Karakuri.Entity) {
    if Karakuri.entity_has_tags(other, "projectile", "player") {
        Karakuri.despawn_entity(self);
    }
}

on_destroy :: (self: *Karakuri.Entity) {
    ok, enemy := Karakuri.get_behavior(self, Enemy);
    assert(ok);

    Karakuri.clear_interval(enemy.shoot_interval);
}
