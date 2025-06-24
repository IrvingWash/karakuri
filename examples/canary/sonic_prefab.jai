#import "Basic";

sonic_prefab :: () -> Karakuri.Entity_Params {
    behavior: *Sonic = New(Sonic);
    behavior.* = .{
        on_start = on_start,
        on_update = on_update,
        rings = 100,
    };

    return .{
        transform = .{
            position = .{100, 100, 0},
            scale = .{2, 2, 1},
        },
        behavior = behavior,
    };
}

Sonic :: struct {
    using #as behavior: Karakuri.Behavior;

    rings: u32;
}

on_start :: (self: *Karakuri.Entity) {
    print("Sonic started\n");
}

on_update :: (self: *Karakuri.Entity, dt: float) {
    print("Sonic updated\n");
}
