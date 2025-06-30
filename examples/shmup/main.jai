#import "Basic"()(MEMORY_DEBUGGER = true, TEMP_ALLOCATOR_POISON_FREED_MEMORY = true);
Karakuri :: #import "Karakuri"()(DEBUG = true);

#load "scene.jai";

main :: () {
    setup_formatting();

    Karakuri.init(.{
        name = "Shmup",
        target_fps = 120,
        scenes = .[
            SCENE,
        ],
        clear_color = .{0.3, 0.3, 0.3, 1},
    });

    Karakuri.run();

    Karakuri.deinit();

    report_memory_leaks();
}

setup_formatting :: () {
    print_style := *context.print_style;
    print_style.default_format_struct.draw_type_name = true;
    print_style.default_format_struct.use_long_form_if_more_than_this_many_members = 0;
    print_style.default_format_struct.use_newlines_if_long_form = true;
}
