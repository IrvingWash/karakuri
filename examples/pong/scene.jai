#load "paddle_prefab.jai";
#load "ball_prefab.jai";

SCENE :: Karakuri.Scene.{
    load = load_scene,
};

load_scene :: () -> [..]Karakuri.Entity_Params {
    result: [..]Karakuri.Entity_Params;

    array_add(
        *result,
        paddle_prefab(),
        ball_prefab(),
    );

    return result;
}
