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
    paddle_token: Karakuri.Entity_Token;
}

#scope_file

on_start :: (self: *Karakuri.Entity) {
    ball := Karakuri.get_behavior(self.*, Ball);
    ball.paddle_token = Karakuri.get_entity_with_name("paddle").token;

    print("Ball started\n");
}

on_update :: (self: *Karakuri.Entity, dt: float) {
    ball := Karakuri.get_behavior(self.*, Ball);

    launch_if_needed(self, ball);
    move(self, ball, dt);

    collide_with_paddle(self, ball);
}

launch_if_needed :: (self: *Karakuri.Entity, using ball: *Ball) {
    if Karakuri.is_key_start(.SPACE) {
        velocity.x, velocity.y = xx -speed;
    }
}

move :: (self: *Karakuri.Entity, using ball: *Ball, dt: float) {
    using self.transform;

    position.x += ball.velocity.x * dt;
    position.y += ball.velocity.y * dt;
}

collide_with_paddle :: (self: *Karakuri.Entity, ball: *Ball) {
    paddle_entity := Karakuri.get_entity(ball.paddle_token);

    if check_aabb_centered(
        self.transform.position.xy,
        paddle_entity.transform.position.xy,
        self.transform.scale.xy,
        paddle_entity.transform.scale.xy
    ) {
        ball.velocity.y *= -1;
    }
}
