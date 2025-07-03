#load "projectile_prefab.jai";

player_prefab :: () -> Karakuri.Entity_Params {
    return .{
        name = "player",
        transform = .{
            position = .{0, -2},
            scale = .{1, 1},
        },
        sprite_component = Karakuri.Maybe.some(
            Karakuri.Sprite_Component.{
                texture_path = "examples/shmup/assets/main_ship-1.png",
                origin = .{0, 0},
            }
        ),
        box_collider_component = Karakuri.Maybe.some(
            Karakuri.Box_Collider_Component.{
                size = .{0.18, 0.44}
            }
        ),
        behavior_component = Karakuri.Maybe.some(
            Karakuri.instantiate_behavior(Player.{})
        ),

        on_start = on_start,
        on_update = on_update,
        on_collision = on_collision,
        on_destroy = on_destroy,
    };
}

Player :: struct {
    using #as behavior: Karakuri.Behavior_Component;

    speed := 5;
    shoot_interval: u32;
}

#scope_file

on_start :: (self: *Karakuri.Entity) {
    ok, player := Karakuri.get_behavior(self, Player);
    assert(ok);

    player.shoot_interval = Karakuri.set_interval(
        1000,
        () {
            Karakuri.spawn_entity(projectile_prefab(.{}));
        }
    );
}

on_update :: (self: *Karakuri.Entity, dt: float) {
    ok, player := Karakuri.get_behavior(self, Player);
    assert(ok);

    move(self, player, dt);
    shoot(self);
}

on_destroy :: (self: *Karakuri.Entity) {
    ok, player := Karakuri.get_behavior(self, Player);
    assert(ok);

    Karakuri.clear_interval(player.shoot_interval);
}

on_collision :: (self: *Karakuri.Entity, other: *Karakuri.Entity) {
    if other.tags.count > 0 && other.tags[0] == "enemy" {
        Karakuri.despawn_entity(self);
    }
}

shoot :: (self: *Karakuri.Entity) {
    if Karakuri.is_key_start(.Space) {
        Karakuri.spawn_entity(projectile_prefab(self.transform.position));
    }
}

move :: (self: *Karakuri.Entity, player: *Player, dt: float) {
    using self.transform;

    if Karakuri.is_key_pressed(.D) || Karakuri.is_key_pressed(.Arrow_Right) {
        position.x += player.speed * dt;
    }
    if Karakuri.is_key_pressed(.A) || Karakuri.is_key_pressed(.Arrow_Left) {
        position.x -= player.speed * dt;
    }
    if Karakuri.is_key_pressed(.W) || Karakuri.is_key_pressed(.Arrow_Up) {
        position.y += player.speed * dt;
    }
    if Karakuri.is_key_pressed(.S) || Karakuri.is_key_pressed(.Arrow_Down) {
        position.y -= player.speed * dt;
    }
}
