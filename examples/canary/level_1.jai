#load "sonic_prefab.jai";

LEVEL_1 :: Karakuri.World.Scene.{
    id = 1,
    load = load_level_1,
};

load_level_1 :: () -> []Karakuri.World.Entity_Params {
    return .[
        sonic_prefab(),
    ];
}
