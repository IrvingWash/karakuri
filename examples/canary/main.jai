#import "Basic"()(MEMORY_DEBUGGER = true);
Karakuri :: #import "Karakuri";

main :: () {
    Karakuri.init(.{
        name = "Canary",
        target_fps = 60,
    });
    defer Karakuri.deinit();

    Karakuri.run();
}
