background_prefab :: () -> Karakuri.Entity_Params {
    return .{
        transform = .{
            scale = .{5.5, 4},
        },
        sprite_component = Karakuri.Maybe.some(
            Karakuri.Sprite_Component.{
                texture_path = "examples/shmup/assets/background.png",
            },
        ),
    };
}
