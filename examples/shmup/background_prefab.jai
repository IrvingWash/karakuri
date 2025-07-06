background_prefab :: () -> Karakuri.Entity_Params {
    return .{
        transform = .{
            scale = .{2, 1.5},
        },
        sprite_component = Karakuri.Maybe.some(
            Karakuri.Sprite_Component.{
                texture_path = "examples/shmup/assets/background.png",
            },
        ),
    };
}
