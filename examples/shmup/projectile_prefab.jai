#import "Math";

Projectile_Emitter :: enum {
    Player;
    Enemy;
}

projectile_prefab :: (position: Vector2, $emitter: Projectile_Emitter) -> Karakuri.Entity_Params {
    width, height := Karakuri.get_render_size();

    tags: [..]string;
    array_add(
        *tags,
        "projectile",
    );

    speed := 8;

    #if emitter == {
        case .Player;
            array_add(*tags, "player");
        case .Enemy;
            array_add(*tags, "enemy");
            speed *= -1;
    }

    return .{
        tags = tags,
        transform = .{
            position = position,
            scale = .{1, 1},
        },
        sprite_component = Karakuri.Maybe.some(
            Karakuri.Sprite_Component.{
                texture_path = "examples/shmup/assets/projectile_2.png",
                flip = .{
                    y = emitter == .Enemy,
                },
                color = ifx emitter == .Player
                    then Vector4.{1, 1, 1, 1}
                    else Vector4.{1, 0.7, 0.3, 1},
            }
        ),
        box_collider_component = Karakuri.Maybe.some(
            Karakuri.Box_Collider_Component.{
                size = .{ 0.17, 0.1},
            }
        ),
        behavior_component = Karakuri.Maybe.some(
            Karakuri.instantiate_behavior(Projectile.{
                emitter = emitter,
                speed = speed
            })
        ),

        on_start = on_start,
        on_update = on_update,
        on_collision = on_collision,
        on_destroy = on_destroy,
    };
}

Projectile :: struct {
    using #as behavior: Karakuri.Behavior_Component;

    speed := 8;
    emitter: Projectile_Emitter;
}

#scope_file

on_start :: (self: *Karakuri.Entity) {}

on_update :: (self: *Karakuri.Entity, dt: float) {
    ok, projectile := Karakuri.get_behavior(self, Projectile);
    assert(ok);

    move(self, projectile, dt);
}

on_collision :: (self: *Karakuri.Entity, other: *Karakuri.Entity) {
    if Karakuri.entity_has_tags(other, "projectile") {
        return;
    }

    ok, projectile := Karakuri.get_behavior(self, Projectile);
    assert(ok);

    if projectile.emitter == {
        case .Player;
            if other.name == "player" {
                return;
            }
        case .Enemy;
            if Karakuri.entity_has_tags(other, "enemy") {
                return;
            }
    }

    Karakuri.despawn_entity(self);
}

on_destroy :: (self: *Karakuri.Entity) {}

move :: (self: *Karakuri.Entity, projectile: *Projectile, dt: float) {
    using self.transform;

    position.y += projectile.speed * dt;
}
