#import "Basic";
#import "Math";
#import "Maybe";

paddle_prefab :: () -> Karakuri.Entity_Params {
    width, height := Karakuri.get_render_size();

    return .{
        transform = .{
            position = .{
                width / 2.0,
                height - (height  - height / 10.0),
                0,
            },
            scale = .{100, 20, 1},
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
    speed := 500;
}

#scope_file

on_start :: (self: *Karakuri.Entity) {
    print("Paddle started\n");
}

on_update :: (self: *Karakuri.Entity, dt: float) {
    paddle := Karakuri.get_behavior(self.*, Paddle);

    move(self, paddle, dt);
}

move :: (self: *Karakuri.Entity, paddle: *Paddle, dt: float) {
    using self.transform;

    if Karakuri.is_key_down(.D) {
        position.x += paddle.speed * dt;
    }
    if Karakuri.is_key_down(.A) {
        position.x -= paddle.speed * dt;
    }
}
