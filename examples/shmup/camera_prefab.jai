camera_prefab :: () -> Karakuri.Entity_Params {
    return .{
        name = "camera",
        behavior_component = Karakuri.Maybe.some(
            Karakuri.instantiate_behavior(Camera.{}),
        ),
        camera_component = Karakuri.Maybe.some(
            Karakuri.Camera_Component.{
                size = 1,
            }
        ),

        on_start = on_start,
        on_update = on_update,
    };
}

Camera :: struct {
    #as using behavior: Karakuri.Behavior_Component;

    player_token: Karakuri.Entity_Token;
    is_targeting := false;
}

on_start :: (self: *Karakuri.Entity) {
    ok, camera := Karakuri.get_behavior(self.*, Camera);
    assert(ok);

    ok =, player := Karakuri.find_entity_with_name("player");
    assert(ok);

    camera.player_token = player.token;
}

on_update :: (self: *Karakuri.Entity, dt: float) {
    ok, camera := Karakuri.get_behavior(self.*, Camera);
    assert(ok);

    ok =, camera_component := Karakuri.get_component(self, Karakuri.Camera_Component);
    assert(ok);

    if Karakuri.is_key_start(.P) {
        camera.is_targeting = !camera.is_targeting;

        if !camera.is_targeting {
            self.transform.position = .{};
            camera_component.size = 1;
        }
    }

    if camera.is_targeting {
        ok =, player := Karakuri.get_entity_with_token(camera.player_token);
        if ok {
            self.transform.position = player.transform.position;
        }

        camera_component.size = 0.7;
    }
}
