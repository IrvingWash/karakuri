init :: (config: Engine_Config) {
    engine = .{
        name = config.name,
        fps_manager = init_fps_manager(config.target_fps),
    };
}

run :: () {
    while true {
        dt := cap_frame_rate(*engine.fps_manager);
    }
}

#scope_file

engine: Engine = ---;

Engine :: struct {
    name: string;
    fps_manager: FPS_Manager;
}

Engine_Config :: struct {
    name: string;
    target_fps: u32 = 60;
}

#import "Basic";
#load "fps_manager.jai";
