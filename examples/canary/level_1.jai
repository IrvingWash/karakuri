#load "sonic_prefab.jai";

LEVEL_1 :: Karakuri.Scene.{
    id = 1,
    load = load_level_1,
};

load_level_1 :: () -> []Karakuri.Entity_Params {
    return .[
        sonic_prefab(),
    ];
}
