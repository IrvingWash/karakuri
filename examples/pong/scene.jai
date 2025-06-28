#load "paddle_prefab.jai";
#load "ball_prefab.jai";
#load "wall_prefab.jai";
#load "camera_prefab.jai";

SCENE :: Karakuri.Scene.{
    load_entities = load_scene,
    load_camera = camera_prefab,
};

load_scene :: () -> [..]Karakuri.Entity_Params {
    result: [..]Karakuri.Entity_Params;

    array_add(
        *result,
        paddle_prefab(),
        ball_prefab(),
        wall_prefab(.Top),
        wall_prefab(.Left),
        wall_prefab(.Right),
        wall_prefab(.Bottom),
    );

    return result;
}
