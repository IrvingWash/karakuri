#import "Basic";
#import "Maybe";

paddle_prefab :: () -> Karakuri.Entity_Params {
    return .{
        transform = .{
            position = .{100, 100, 0},
            scale = .{100, 30, 1},
        },
        sprite_component = some(
            Karakuri.Sprite_Component.{
                color = .{0, 0, 1, 1},
            }
        ),
        on_start = on_start,
        on_update = on_update,
    };
}

Paddle :: struct {
    speed := 100;
}

on_start :: (self: *Karakuri.Entity) {
    print("Paddle started\n");
}

on_update :: (self: *Karakuri.Entity, dt: float) {}
