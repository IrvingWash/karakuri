ball_prefab :: () -> Karakuri.Entity_Params {
    width, height := Karakuri.get_render_size();

    return .{
        transform = .{
            position = .{
                width / 2.0,
                height / 2.0,
                0,
            },
            scale = .{20, 20, 1},
        },
        sprite_component = some(
            Karakuri.Sprite_Component.{
                color = .{1, 0, 0, 1},
            }
        ),
        behavior_component = some(
            Karakuri.instantiate_behavior(Ball, Ball.{})
        ),
        on_start = on_start,
        on_update = on_update,
    };
}

Ball :: struct {
    #as using behavior: Karakuri.Behavior_Component;
    speed := 700;
    velocity: Vector2;
}

#scope_file

on_start :: (self: *Karakuri.Entity) {
    print("Ball started\n");
}

on_update :: (self: *Karakuri.Entity, dt: float) {
    ball := Karakuri.get_behavior(self.*, Ball);

    launch_if_needed(self, ball);
    move(self, ball, dt);
    collide_with_walls(self, *ball.velocity);
}

launch_if_needed :: (self: *Karakuri.Entity, using ball: *Ball) {
    if Karakuri.is_key_start(.SPACE) {
        velocity.x, velocity.y = xx speed;
    }
}

move :: (self: *Karakuri.Entity, using ball: *Ball, dt: float) {
    self.transform.position.x += ball.velocity.x * dt;
    self.transform.position.y += ball.velocity.y * dt;
}

collide_with_walls :: (self: *Karakuri.Entity, velocity: *Vector2) {
    using self.transform;

    width, height := Karakuri.get_render_size();

    half_scale := scale.x / 2;
    left_coordinate := position.x - half_scale;
    right_coordinate := position.x + half_scale;
    top_coordinate := position.y + half_scale;
    bottom_coordinate := position.y - half_scale;

    if (left_coordinate) <= 0 || right_coordinate >= xx width {
        velocity.x *= -1;
    }

    if (top_coordinate) >= xx height || bottom_coordinate <= 0 {
        velocity.y *= -1;
    }
}
