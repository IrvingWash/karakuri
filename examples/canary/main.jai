#import "Basic";
Karakuri :: #import "Karakuri";

main :: () {
    Karakuri.init(.{
        name = "Canary",
        target_fps = 60,
    });

    Karakuri.run();
}
