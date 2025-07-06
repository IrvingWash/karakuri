enemy_spawner_prefab :: () -> Karakuri.Entity_Params {
    return .{
        behavior_component = Karakuri.Maybe.some(
            Karakuri.instantiate_behavior(Enemy_Spawner.{}),
        ),

        on_start = on_start,
    };
}

Enemy_Spawner :: struct {
    using #as behavior: Karakuri.Behavior_Component;
    spawn_interval: float64 = 1000;
}

#scope_file

on_start :: (self: *Karakuri.Entity) {
    ok, spawner := Karakuri.get_behavior(self, Enemy_Spawner);
    assert(ok);

    interval_data := New(Karakuri.Entity_Token);
    interval_data.* = self.token;
    Karakuri.set_interval(
        spawner.spawn_interval,
        (token: *Karakuri.Entity_Token) {
            ok, self := Karakuri.get_entity_with_token(token);
            assert(ok);

            Karakuri.spawn_entity(
                enemy_prefab(.{
                    Karakuri.random_in_range(-5, 5),
                    Karakuri.random_in_range(2, 5)
                })
            );
        },
        interval_data,
        should_cleanup = true
    );
}
