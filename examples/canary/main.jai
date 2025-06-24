#import "Basic"()(MEMORY_DEBUGGER = true);
Karakuri :: #import "Karakuri";

#load "level_1.jai";

main :: () {
    setup_formatting();

    scenes: [..]Karakuri.Scene;
    array_add(*scenes, LEVEL_1);

    Karakuri.init(.{
        name = "Canary",
        target_fps = 60,
        scenes = scenes,
        initial_scene_id = 1,
    });
    defer Karakuri.deinit();

    Karakuri.run();
}

setup_formatting :: () {
    print_style := *context.print_style;
    print_style.default_format_struct.draw_type_name = true;
    print_style.default_format_struct.use_long_form_if_more_than_this_many_members = 0;
    print_style.default_format_struct.use_newlines_if_long_form = true;
}
