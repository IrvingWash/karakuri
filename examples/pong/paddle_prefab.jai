#import "Basic";
#import "Math";
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
    speed := 500;
}

on_start :: (self: *Karakuri.Entity) {
    print("Paddle started\n");
}

on_update :: (self: *Karakuri.Entity, dt: float) {
    paddle := Karakuri.get_behavior(self.*, Paddle);

    move(self, paddle, dt);
    collide_with_walls(self);
}

move :: (self: *Karakuri.Entity, paddle: *Paddle, dt: float) {
    if Karakuri.is_key_down(.D) {
        self.transform.position.x += paddle.speed * dt;
    }
    if Karakuri.is_key_down(.A) {
        self.transform.position.x -= paddle.speed * dt;
    }
}

collide_with_walls :: (self: *Karakuri.Entity) {
    using self.transform;

    width := Karakuri.get_render_size();

    half_scale := scale.x / 2;
    left_coordinate := position.x - half_scale;
    right_coordinate := position.x + half_scale;

    if (left_coordinate) <= 0 {
        position.x = half_scale;
    }
    if right_coordinate >= xx width {
        position.x = width - half_scale;
    }
}
