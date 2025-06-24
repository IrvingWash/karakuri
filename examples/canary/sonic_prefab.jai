#import "Basic";

sonic_prefab :: () -> Karakuri.Entity_Params {
    return .{
        transform = .{
            position = .{100, 100, 0},
            scale = .{100, 100, 1},
        },
        sprite = .{color = .{1, 0, 0, 1}},
        on_start = on_start,
        on_update = on_update,
        behavior = Karakuri.instantiate_behavior(
            Sonic,
            .{rings = 100}
        ),
    };
}

Sonic :: struct {
    using #as behavior: Karakuri.Behavior;

    rings: u32;
}

on_start :: (self: *Karakuri.Entity) {
    print("Sonic started\n");
}

on_update :: (using self: *Karakuri.Entity, dt: float) {
    transform.position.x += 100 * dt;
    transform.position.y += 100 * dt;
}
