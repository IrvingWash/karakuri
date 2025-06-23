#import "Basic";
#import,dir "platform";

init :: (config: Engine_Config) {
    init_fps_manager(config.target_fps);

    init_window_manager(
        config.window_size.width,
        config.window_size.height,
        config.name
    );

    World.init_world();

    engine = .{
        scenes = config.scenes,
    };

    initial_scene: Scene;
    for scene: engine.scenes {
        if scene.id == config.initial_scene_id {
            initial_scene = scene;
            break;
        }
    }

    World.set_initial_entities(initial_scene.load());
}

run :: () {
    while true {
        dt := cap_frame_rate();

        events := process_input();
        if events.quit {
            break;
        }
    }
}

deinit :: () {}

#scope_file

process_input :: () -> Events {
    return get_events();
}

engine: Engine = ---;

Engine :: struct {
    scenes: []World.Scene;
}

Engine_Config :: struct {
    name: string;
    target_fps: u32 = 60;
    window_size: struct {
        width: u32 = 800;
        height: u32 = 600;
    };
    scenes: []World.Scene;
    initial_scene_id: u32;
}
