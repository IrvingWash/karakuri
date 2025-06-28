camera_prefab :: () -> Karakuri.Camera_Params {
    return .{
        size = 0.9,
        behavior = Karakuri.instantiate_behavior(Camera.{}),
        on_start = (self: *Karakuri.Camera) {
            ok, camera := Karakuri.get_downcasted_behavior(self.*, Camera);
            assert(ok);

            ok =, ball := Karakuri.find_entity_with_name("ball");
            assert(ok);

            camera.ball_token = ball.token;
        },
        on_update = (self: *Karakuri.Camera, dt: float) {
            ok, camera := Karakuri.get_downcasted_behavior(self.*, Camera);
            assert(ok);

            ok =, ball := Karakuri.get_entity_with_token(camera.ball_token);
            assert(ok);

            self.position = ball.transform.position;
        },
    };
}

Camera :: struct {
    #as using behavior: Karakuri.Behavior_Component;

    ball_token: Karakuri.Entity_Token;
}
