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
            scale = .{9 * 2, 34 * 2},
        },
        sprite_component = Karakuri.Maybe.some(
            Karakuri.Sprite_Component.{
                texture_path = "examples/shmup/assets/projectile_1.png",
            }
        ),
        behavior_component = Karakuri.Maybe.some(
            Karakuri.instantiate_behavior(Projectile.{})
        ),

        on_start = on_start,
        on_update = on_update,
        on_destroy = on_destroy,
    };
}

Projectile :: struct {
    using #as behavior: Karakuri.Behavior_Component;

    speed := 800;
}

#scope_file

on_start :: (self: *Karakuri.Entity) {
    print("Projectile started\n");
}

on_update :: (self: *Karakuri.Entity, dt: float) {
    ok, projectile := Karakuri.get_behavior(self.*, Projectile);
    assert(ok);

    move(self, projectile, dt);
}

on_destroy :: (self: *Karakuri.Entity) {
    print("Projectile destroyed\n");
}

move :: (self: *Karakuri.Entity, projectile: *Projectile, dt: float) {
    using self.transform;

    position.y += projectile.speed * dt;
}
