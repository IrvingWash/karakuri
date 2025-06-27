Wall_Position :: enum {
    Top;
    Right;
    Left;
    Bottom;
}

wall_prefab :: ($position: Wall_Position) -> Karakuri.Entity_Params {
    width, height := Karakuri.get_render_size();

    transform: Karakuri.Transform_Component;

    #if #complete position == {
        case .Top;
            transform = .{
                position = .{
                    width / 2.0,
                    height - 5.0,
                    0,
                },
                scale = .{xx width, 1, 1},
            };
        case .Right;
            transform = .{
                position = .{
                    width - 5.0,
                    height / 2.0,
                    0,
                },
                scale = .{1, xx height, 1},
            };
        case .Left;
            transform = .{
                position = .{
                    0 + 5.0,
                    height / 2.0,
                    0,
                },
                scale = .{1, xx height, 1},
            };
        case .Bottom;
            transform = .{
                position = .{
                    width / 2.0,
                    0 + 5.0,
                    0,
                },
                scale = .{xx width, 1, 1},
            };
    }

    tags: [..]string;
    array_add(
        *tags,
        "wall",
        "test",
    );

    return .{
        tags = tags,
        transform = transform,
        behavior_component = Karakuri.Maybe.some(
            Karakuri.instantiate_behavior(
                Wall,
                .{
                    position = position,
                }
            )
        ),
        on_start = on_start,
        on_update = on_update,
    };
}

Wall :: struct {
    #as using behavior: Karakuri.Behavior_Component;
    ball_token: Karakuri.Entity_Token;
    paddle_token: Karakuri.Entity_Token;
    position: Wall_Position;
}

#scope_file

on_start :: (self: *Karakuri.Entity) {
    ok, wall := Karakuri.get_behavior(self, Wall);
    if !ok {
        exit(1);
    }

    ok =, ball_entity := Karakuri.get_entity_with_name("ball");
    if !ok {
        exit(1);
    }

    ok =, paddle_entity := Karakuri.get_entity_with_name("paddle");
    if !ok {
        exit(1);
    }

    wall.ball_token = ball_entity.token;
    wall.paddle_token = paddle_entity.token;
}

on_update :: (self: *Karakuri.Entity, dt: float) {
    ok, wall := Karakuri.get_behavior(self, Wall);
    if !ok {
        exit(1);
    }

    ok =, ball_entity := Karakuri.get_entity_with_token(wall.ball_token);
    if !ok {
        exit(1);
    }
    ok =, ball := Karakuri.get_behavior(ball_entity, Ball);
    if !ok {
        exit(1);
    }

    ok =, paddle_entity := Karakuri.get_entity_with_token(wall.paddle_token);
    if !ok {
        exit(1);
    }

    width, height := Karakuri.get_render_size();

    if check_aabb_centered(
        self.transform.position.xy,
        ball_entity.transform.position.xy,
        self.transform.scale.xy,
        ball_entity.transform.scale.xy
    ) {
        if wall.position == {
            case .Top;
                #through;
            case .Bottom;
                ball.velocity.y *= -1;
            case .Right;
                #through;
            case .Left;
                ball.velocity.x *= -1;
        }
    };

    if (
        (wall.position == .Right || wall.position == .Left)
        && check_aabb_centered(
            self.transform.position.xy,
            paddle_entity.transform.position.xy,
            self.transform.scale.xy,
            paddle_entity.transform.scale.xy
        )
    ) {
        if wall.position == {
            case .Right;
                paddle_entity.transform.position.x = self.transform.position.x - self.transform.scale.x / 2 - paddle_entity.transform.scale.x / 2;
            case .Left;
                paddle_entity.transform.position.x = self.transform.position.x + self.transform.scale.x / 2 + paddle_entity.transform.scale.x / 2;
        }
    };
}
