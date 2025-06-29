player_prefab :: () -> Karakuri.Entity_Params {
    width, height := Karakuri.get_render_size();

    return .{
        name = "paddle",
        transform = .{
            scale = .{48 * 2, 58 * 2},
        },
        sprite_component = Karakuri.Maybe.some(
            Karakuri.Sprite_Component.{
                texture_path = "examples/shmup/assets/main_ship-1.png",
            }
        ),
        behavior_component = Karakuri.Maybe.some(
            Karakuri.instantiate_behavior(Player.{})
        ),

        on_update = on_update,
    };
}

Player :: struct {
    using #as behavior: Karakuri.Behavior_Component;

    speed := 500;
}

#scope_file

on_update :: (self: *Karakuri.Entity, dt: float) {
    ok, player := Karakuri.get_behavior(self.*, Player);
    assert(ok);

    move(self, player, dt);
}

move :: (self: *Karakuri.Entity, player: *Player, dt: float) {
    using self.transform;

    if Karakuri.is_key_pressed(.D) {
        position.x += player.speed * dt;
    }
    if Karakuri.is_key_pressed(.A) {
        position.x -= player.speed * dt;
    }
    if Karakuri.is_key_pressed(.W) {
        position.y += player.speed * dt;
    }
    if Karakuri.is_key_pressed(.S) {
        position.y -= player.speed * dt;
    }
}
