enemy_spawner_prefab :: () -> Karakuri.Entity_Params {
    return .{
        behavior_component = Karakuri.Maybe.some(
            Karakuri.instantiate_behavior(Enemy_Spawner.{}),
        ),

        on_start = on_start,
        on_destroy = on_destroy,
    };
}

Enemy_Spawner :: struct {
    using #as behavior: Karakuri.Behavior_Component;
    spawn_interval: float64 = 1000;
    spawn_interval_id: u32;
}

#scope_file

on_start :: (self: *Karakuri.Entity) {
    ok, spawner := Karakuri.get_behavior(self, Enemy_Spawner);
    assert(ok);

    interval_data := New(Karakuri.Entity_Token);
    interval_data.* = self.token;
    spawner.spawn_interval_id = Karakuri.set_interval(
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

on_destroy :: (self: *Karakuri.Entity) {
    ok, spawner := Karakuri.get_behavior(self, Enemy_Spawner);
    assert(ok);

    Karakuri.clear_interval(spawner.spawn_interval_id);
}
