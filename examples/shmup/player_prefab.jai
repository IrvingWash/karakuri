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

        on_update = on_update,
        on_collision = on_collision,
    };
}

Player :: struct {
    using #as behavior: Karakuri.Behavior_Component;

    speed := 5;
    lives := 3;
}

#scope_file

on_update :: (self: *Karakuri.Entity, dt: float) {
    ok, player := Karakuri.get_behavior(self, Player);
    assert(ok);

    move(self, player, dt);
    shoot(self);
}

on_collision :: (self: *Karakuri.Entity, other: *Karakuri.Entity) {
    if !Karakuri.entity_has_tags(other, "enemy") {
        return;
    }

    ok, player := Karakuri.get_behavior(self, Player);

    if player.lives > 1 {
        player.lives -= 1;
    } else {
        Karakuri.despawn_entity(self);
    }
}

shoot :: (self: *Karakuri.Entity) {
    if Karakuri.is_key_start(.Space) {
        Karakuri.spawn_entity(projectile_prefab(self.transform.position, .Player));
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
