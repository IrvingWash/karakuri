init :: (config: Engine_Config) {
    engine = .{
        name = config.name,
        fps_manager = init_fps_manager(config.target_fps),
        window_manager = init_window_manager(
            config.window_size.width,
            config.window_size.height,
            config.name
        ),
    };
}

run :: () {
    while !engine.window_manager.events.quit {
        dt := cap_frame_rate(*engine.fps_manager);

        get_events(*engine.window_manager);
    }
}

deinit :: () {}

#scope_file

engine: Engine = ---;

Engine :: struct {
    name: string;
    fps_manager: FPS_Manager;
    window_manager: Window_Manager;
}

Engine_Config :: struct {
    name: string;
    target_fps: u32 = 60;
    window_size: struct {
        width: u32 = 800;
        height: u32 = 600;
    };
}

#import "Basic";
#load "fps_manager.jai";
#load "window_manager.jai";
