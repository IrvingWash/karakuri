#import "Basic"()(MEMORY_DEBUGGER = true);
Karakuri :: #import "Karakuri";
#load "level_1.jai";

main :: () {
    Karakuri.init(.{
        name = "Canary",
        target_fps = 60,
        scenes = .[
            LEVEL_1,
        ],
        initial_scene_id = 1,
    });
    defer Karakuri.deinit();

    Karakuri.run();
}
