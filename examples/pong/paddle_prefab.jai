#import "Basic";
#import "Math";

paddle_prefab :: () -> Karakuri.Entity_Params {
    width, height := Karakuri.get_render_size();

    return .{
        name = "paddle",
        transform = .{
            position = .{
                width / 2.0,
                height - (height  - height / 10.0),
            },
            scale = .{100, 20},
        },
        sprite_component = Karakuri.Maybe.some(
            Karakuri.Sprite_Component.{
                // origin = .{0.5, 0.5},
                color = .{0, 0, 1, 1},
            }
        ),
        behavior_component = Karakuri.Maybe.some(
            Karakuri.instantiate_behavior(Paddle, Paddle.{})
        ),
        on_start = on_start,
        on_update = on_update,
    };
}

Paddle :: struct {
    #as using behavior: Karakuri.Behavior_Component;
    speed := 500;
}

#scope_file

on_start :: (self: *Karakuri.Entity) {
    print("Paddle started\n");
}

on_update :: (self: *Karakuri.Entity, dt: float) {
    ok, paddle := Karakuri.get_behavior(self.*, Paddle);
    if !ok {
        exit(1);
    }

    move(self, paddle, dt);
}

move :: (self: *Karakuri.Entity, paddle: *Paddle, dt: float) {
    using self.transform;

    if Karakuri.is_key_pressed(.D) {
        position.x += paddle.speed * dt;
    }
    if Karakuri.is_key_pressed(.A) {
        position.x -= paddle.speed * dt;
    }
}
