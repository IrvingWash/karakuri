#load "player_prefab.jai";

SCENE :: Karakuri.Scene.{
    load_entities = load_scene,
};

load_scene :: () -> [..]Karakuri.Entity_Params {
    result: [..]Karakuri.Entity_Params;

    array_add(
        *result,
        player_prefab(),
    );

    return result;
}
