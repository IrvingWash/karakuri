camera_prefab :: () -> Karakuri.Camera_Params {
    return .{
        size = 1,
        behavior = Karakuri.instantiate_behavior(Camera.{}),
        on_start = on_start,
        on_update = on_update,
    };
}

Camera :: struct {
    #as using behavior: Karakuri.Behavior_Component;

    player_token: Karakuri.Entity_Token;
    is_targeting := false;
}

on_start :: (self: *Karakuri.Camera) {
    ok, camera := Karakuri.get_behavior(self.*, Camera);
    assert(ok);

    ok =, player := Karakuri.find_entity_with_name("player");
    assert(ok);

    camera.player_token = player.token;
}

on_update :: (self: *Karakuri.Camera, dt: float) {
    ok, camera := Karakuri.get_behavior(self.*, Camera);
    assert(ok);

    if Karakuri.is_key_start(.P) {
        camera.is_targeting = !camera.is_targeting;

        if !camera.is_targeting {
            self.position = .{};
            self.size = 1;
        }
    }

    if camera.is_targeting {
        ok =, player := Karakuri.get_entity_with_token(camera.player_token);
        if ok {
            self.position = player.transform.position;
        }

        self.size = 0.7;
    }
}
