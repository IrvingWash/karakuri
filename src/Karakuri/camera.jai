#import "Math";

Camera :: struct {
    using params: Camera_Params;

    front :: Vector3.{0, 0, -1};
    up :: Vector3.{0, 1, 0};
    right :: Vector3.{1, 1, 0};
}

Camera_Params :: struct {
    position: Vector2;
    size: float = 1;
    behavior: *Behavior_Component;

    on_start: (self: *Camera);
    on_update: (self: *Camera, dt: float);
}

#scope_module

camera_get_behavior :: (camera: Camera, $B: Type) -> ok: bool, *B
#modify { return uses_behavior_with_as(B); } {
    return camera.behavior != null, cast(*B) camera.behavior;
}

create_camera :: (params: Camera_Params) -> Camera {
    return .{params = params};
}

start_camera :: (camera: *Camera) {
    if camera.on_start {
        camera.on_start(camera);
    }
}

update_camera :: (camera: *Camera, dt: float) {
    if camera.on_update {
        camera.on_update(camera, dt);
    }
}

destroy_camera :: (camera: *Camera) {
    camera.on_start = null;
    camera.on_update = null;

    if camera.behavior {
        free(camera.behavior);
    }
}

make_view_matrix :: (camera: Camera) -> Matrix4 {
    position := Vector3.{
        camera.position.x * PIXELS_PER_UNIT,
        camera.position.y * PIXELS_PER_UNIT,
        0,
    };

    return make_look_at_matrix(position, position + camera.front, camera.up, false);
}
