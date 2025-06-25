Wall_Position :: enum {
    Top;
    Right;
    Left;
    Bottom;
}

wall_prefab :: ($position: Wall_Position) -> Karakuri.Entity_Params {
    width, height := Karakuri.get_render_size();

    transform: Karakuri.Transform_Component;

    #if position == {
        case .Top;
            transform = .{
                position = .{
                    width / 2.0,
                    height - 5.0,
                    0,
                },
                scale = .{xx width, 10, 1},
            };
        case .Right;
            transform = .{
                position = .{
                    width - 5.0,
                    height / 2.0,
                    0,
                },
                scale = .{10, xx height, 1},
            };
        case .Left;
            transform = .{
                position = .{
                    0 + 5.0,
                    height / 2.0,
                    0,
                },
                scale = .{10, xx height, 1},
            };
        case .Bottom;
            transform = .{
                position = .{
                    width / 2.0,
                    0 + 5.0,
                    0,
                },
                scale = .{xx width, 10, 1},
            };
    }

    return .{
        transform = transform,
        sprite_component = some(
            Karakuri.Sprite_Component.{
                color = .{0, 0, 0, 1},
            },
        ),
    };
}
