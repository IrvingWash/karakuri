#load "player_prefab.jai";
#load "enemy_prefab.jai";
#load "camera_prefab.jai";

SCENE :: Karakuri.Scene.{
    load_entities = load_scene,
    load_camera = camera_prefab,
};

load_scene :: () -> [..]Karakuri.Entity_Params {
    result: [..]Karakuri.Entity_Params;

    array_add(
        *result,
        player_prefab(),
        enemy_prefab(),
    );

    return result;
}
