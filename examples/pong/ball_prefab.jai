ball_prefab :: () -> Karakuri.Entity_Params {
    width, height := Karakuri.get_render_size();

    return .{
        name = "ball",
        transform = .{
            position = .{
                width / 2.0,
                height / 2.0,
                0,
            },
            scale = .{20, 20, 1},
        },
        sprite_component = Karakuri.Maybe.some(
            Karakuri.Sprite_Component.{
                color = .{1, 0, 0, 1},
            }
        ),
        behavior_component = Karakuri.Maybe.some(
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
    ok, ball := Karakuri.get_behavior(self.*, Ball);
    if !ok {
        exit(1);
    }

    ok =, paddle_entity := Karakuri.get_entity_with_name("paddle");
    if !ok {
        exit(1);
    }

    ball.paddle_token = paddle_entity.token;

    print("Ball started\n");
}

on_update :: (self: *Karakuri.Entity, dt: float) {
    ok, ball := Karakuri.get_behavior(self.*, Ball);
    if !ok {
        exit(1);
    }

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
    ok, paddle_entity := Karakuri.get_entity_with_token(ball.paddle_token);
    if !ok {
        exit(1);
    }

    if check_aabb_centered(
        self.transform.position.xy,
        paddle_entity.transform.position.xy,
        self.transform.scale.xy,
        paddle_entity.transform.scale.xy
    ) {
        ball.velocity.y *= -1;
    }
}
