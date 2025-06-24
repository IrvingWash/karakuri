#import "Basic";
#import "FPS_Manager";
#import "Window_Manager";
#import "Input_Manager";
#import "Sprite_Renderer";

init :: (config: Engine_Config) {
    init_fps_manager(config.target_fps);

    window := init_window_manager(
        config.window_size.width,
        config.window_size.height,
        config.name
    );

    init_sprite_renderer(window, config.clear_color);

    engine = .{
        scenes = config.scenes,
    };

    if engine.scenes.count == 0 {
        print("No scenes provided. Aborting\n");
        exit(1);
    }

    initial_scene: Scene;
    for scene: engine.scenes {
        if scene.id == config.initial_scene_id {
            initial_scene = scene;
            break;
        }
    }

    init_world();

    set_entities(initial_scene.load());
}

run :: () {
    while !engine.should_terminate {
        dt := cap_frame_rate();

        events := process_input();

        update(dt);

        render();

        reset_temporary_storage();
    }
}

deinit :: () {}

get_render_size :: get_render_dimensions;

#scope_file

process_input :: () -> Events {
    events := get_events();

    if events.quit {
        engine.should_terminate = true;
    }

    for events.window_resizes {
        update_window();
    }

    return events;
}

update :: (dt: float) {
    update_world(dt);
}

render :: () {
    start_drawing();

    entities_with_sprite := get_entities_with_components(Sprite_Component);
    for entity: entities_with_sprite {
        sprite := get_component(entity, Sprite_Component);

        draw_sprite(
            entity.transform.position,
            entity.transform.scale,
            sprite.color,
        );
    }

    finish_drawing();
}

engine: Engine = ---;

Engine :: struct {
    scenes: [..]Scene; // @Todo try to make these []
    should_terminate: bool;
}

Engine_Config :: struct {
    name: string;
    target_fps: u32 = 60;
    window_size: struct {
        width: u32 = 800;
        height: u32 = 600;
    };
    scenes: [..]Scene;
    initial_scene_id: u32;
    clear_color: Vector4;
}
