#import "Basic";
#import,dir "platform";
Sprite_Renderer :: #import,dir "sprite_renderer";

init :: (config: Engine_Config) {
    init_fps_manager(config.target_fps);

    window := init_window_manager(
        config.window_size.width,
        config.window_size.height,
        config.name
    );

    Sprite_Renderer.init(window);

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
    while !engine.should_terminate {
        dt := cap_frame_rate();

        events := process_input();

        update(dt);

        render();
    }
}

deinit :: () {}

#scope_file

process_input :: () -> Events {
    events := get_events();

    if events.quit {
        engine.should_terminate = true;
    }

    for events.window_resizes {
        Sprite_Renderer.update_window();
    }

    return events;
}

update :: (dt: float) {
    World.update(dt);
}

render :: () {
    Sprite_Renderer.start_drawing();

    for entity: World.entities() {
        Sprite_Renderer.draw_sprite(
            entity.transform.position,
            entity.transform.scale,
            entity.sprite.color,
        );
    }

    Sprite_Renderer.finish_drawing();
}

engine: Engine = ---;

Engine :: struct {
    scenes: [..]World.Scene;
    should_terminate: bool;
}

Engine_Config :: struct {
    name: string;
    target_fps: u32 = 60;
    window_size: struct {
        width: u32 = 800;
        height: u32 = 600;
    };
    scenes: [..]World.Scene;
    initial_scene_id: u32;
}
