enemy_prefab :: () -> Karakuri.Entity_Params {
    return .{
        transform = .{
            position = .{0, 200},
            scale = .{48, 48},
        },
        sprite_component = Karakuri.Maybe.some(Karakuri.Sprite_Component.{
            texture_path = "examples/shmup/assets/enemy_2_1.png",
        }),
    };
}
