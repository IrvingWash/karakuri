#load "paddle_prefab.jai";

SCENE :: Karakuri.Scene.{
    load = load_scene,
};

load_scene :: () -> [..]Karakuri.Entity_Params {
    result: [..]Karakuri.Entity_Params;

    array_add(
        *result,
        paddle_prefab(),
    );

    return result;
}
