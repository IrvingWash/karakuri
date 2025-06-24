#import "Basic";

sonic_prefab :: () -> Karakuri.Entity_Params {
    return .{
        transform = .{
            position = .{100, 100, 0},
            scale = .{2, 2, 1},
        },
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

on_update :: (self: *Karakuri.Entity, dt: float) {
    _, sonic := Karakuri.downcast_behavior(self.*, Sonic);
    print("Sonic has % rings\n", sonic.rings);
}
