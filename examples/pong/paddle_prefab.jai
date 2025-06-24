#import "Basic";
#import "Maybe";

paddle_prefab :: () -> Karakuri.Entity_Params {
    width, height := Karakuri.get_render_size();
    scale_x: float = 100;
    scale_y: float = 20;

    return .{
        transform = .{
            position = .{
                width / 2.0,
                height - (height  - height / 10.0),
                0,
            },
            scale = .{scale_x, scale_y, 1},
        },
        sprite_component = some(
            Karakuri.Sprite_Component.{
                color = .{0, 0, 1, 1},
            }
        ),
        behavior_component = some(
            Karakuri.instantiate_behavior(Paddle, Paddle.{})
        ),
        on_start = on_start,
        on_update = on_update,
    };
}

Paddle :: struct {
    #as using behavior: Karakuri.Behavior_Component;
    speed := 100;
}

on_start :: (self: *Karakuri.Entity) {
    print("Paddle started\n");
}

on_update :: (self: *Karakuri.Entity, dt: float) {
    paddle := Karakuri.get_behavior(self.*, Paddle);

    // self.transform.position.x += paddle.speed * dt;
}
