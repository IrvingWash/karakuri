#load "player_prefab.jai";
#load "enemy_prefab.jai";
#load "camera_prefab.jai";
#load "enemy_spawner_prefab.jai";
#load "projectile_destroyer_prefab.jai";

SCENE :: Karakuri.Scene.{
    load_entities = load_scene,
};

load_scene :: () -> [..]Karakuri.Entity_Params {
    result: [..]Karakuri.Entity_Params;

    array_add(
        *result,
        camera_prefab(),
        player_prefab(),
        enemy_spawner_prefab(),
        projectile_destroyer_prefab(.Top),
        projectile_destroyer_prefab(.Bottom),
    );

    return result;
}
