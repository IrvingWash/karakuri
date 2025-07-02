#import "Math";

projectile_prefab :: (position: Vector2) -> Karakuri.Entity_Params {
    width, height := Karakuri.get_render_size();

    tags: [..]string;
    array_add(
        *tags,
        "projectile",
    );

    return .{
        tags = tags,
        transform = .{
            position = position,
            scale = .{1, 1},
        },
        sprite_component = Karakuri.Maybe.some(
            Karakuri.Sprite_Component.{
                texture_path = "examples/shmup/assets/projectile_1.png",
            }
        ),
        box_collider_component = Karakuri.Maybe.some(
            Karakuri.Box_Collider_Component.{}
        ),
        behavior_component = Karakuri.Maybe.some(
            Karakuri.instantiate_behavior(Projectile.{})
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
}

#scope_file

on_start :: (self: *Karakuri.Entity) {}

on_update :: (self: *Karakuri.Entity, dt: float) {
    ok, projectile := Karakuri.get_behavior(self.*, Projectile);
    assert(ok);

    move(self, projectile, dt);
}

on_collision :: (self: *Karakuri.Entity, other: *Karakuri.Entity) {
    if other.name == "player"
    || (other.tags.count > 0 && other.tags[0] == "projectile")
    {
        return;
    }

    Karakuri.despawn_entity(self);
}

on_destroy :: (self: *Karakuri.Entity) {}

move :: (self: *Karakuri.Entity, projectile: *Projectile, dt: float) {
    using self.transform;

    position.y += projectile.speed * dt;
}
