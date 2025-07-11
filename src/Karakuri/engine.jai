using Basic :: #import "Basic";
#poke_name Basic operator ==;

init :: (config: Engine_Config) {
    FPS_Manager.init(config.target_fps);

    ok, window := Window_Manager.init(
        config.window_size.width,
        config.window_size.height,
        config.name
    );
    if !ok {
        log_error("Failed to initialize window");
        exit(1);
    }

    Input_Manager.init();

    Sprite_Renderer.init(window, config.clear_color, PIXELS_PER_UNIT);

    engine = .{
        scenes = config.scenes,
    };

    if engine.scenes.count == 0 {
        log_error("No scenes provided. Aborting");
        exit(1);
    }

    initial_scene: Scene;
    for scene: engine.scenes {
        if scene.id == config.initial_scene_id {
            initial_scene = scene;
            break;
        }
    }
    set_scene(initial_scene);
}

run :: () {
    while !engine.should_terminate {
        dt := FPS_Manager.cap_frame_rate();

        process_input();

        update(dt);

        render();

        reset_temporary_storage();

        #if UPDATE_MEMORY_VISUALIZER {
            memory_visualizer_per_frame_update();
        }
    }
}

deinit :: () {
    deinit_world();
    Asset_Manager.deinit();
    Sprite_Renderer.deinit();
    Timer.deinit();
}

#scope_module

Input_Manager:   : #import "Input_Manager";
Sprite_Renderer: : #import "Sprite_Renderer";
Timer:           : #import "Timer";

PIXELS_PER_UNIT :: 100;

#scope_file

set_scene :: (scene: Scene) {
    Timer.deinit();
    Asset_Manager.deinit();
    deinit_world();

    Timer.init();
    Asset_Manager.init();
    init_world();

    #if DEBUG {
        Asset_Manager.add_texture("src/Karakuri/assets/textures/square.png");
    }

    world_set_entities(scene);
}

process_input :: () {
    events := Input_Manager.get_events();

    if events.quit {
        engine.should_terminate = true;
    }

    for events.window_resizes {
        Sprite_Renderer.update_window();
    }
}

update :: (dt: float) {
    Timer.update(seconds_since_init() * 1000);

    physics_system();

    update_world(dt);
}

render :: () {
    entities_with_camera := world_find_entities_with_components(Camera_Component);
    ok, camera           := world_get_component(entities_with_camera[0], Camera_Component); // @Todo currently using only the first camera
    assert(ok);
    Sprite_Renderer.start_drawing(
        make_scale_matrix4(Vector3.{camera.size, camera.size, 1})
        * make_view_matrix(entities_with_camera[0].transform, camera),
    );

    entities_with_sprite := world_find_entities_with_components(Sprite_Component);
    for entity: entities_with_sprite {
        ok =, sprite := world_get_component(entity, Sprite_Component);
        if !ok {
            log_error(
                "Component % for entity $ was not found though it should have been",
                Sprite_Component,
                entity.token,
            );

            continue;
        }

        Sprite_Renderer.draw(
            entity.transform.position,
            entity.transform.scale,
            entity.transform.rotation,
            sprite.origin,
            sprite.color,
            sprite.pixels_per_unit,
            sprite.texture,
            sprite.flip,
        );
    }

    #if DEBUG {
        // @Todo do something with this flip
        Flip:                       : struct {x: bool; y: bool;};
        flip:                       : Flip.{};
        GREEN:                      : Vector4.{0, 1, 0, 0.2};
        ok =, collider_texture:     = Asset_Manager.get_texture("src/Karakuri/assets/textures/square.png");
        assert(ok);
        entities_with_box_collider: = world_find_entities_with_components(Box_Collider_Component);
        for entity: entities_with_box_collider {
            ok =, box_collider := world_get_component(entity, Box_Collider_Component);
            if !ok {
                log_error(
                    "Component % for entity $ was not found though it should have been",
                    Box_Collider_Component,
                    entity.token,
                );

                continue;
            }

            Sprite_Renderer.draw(
                entity.transform.position,
                entity.transform.scale * box_collider.size,
                entity.transform.rotation,
                box_collider.offset,
                GREEN,
                PIXELS_PER_UNIT,
                collider_texture,
                flip,
            );
        }
    }

    Sprite_Renderer.finish_drawing();
}

engine: Engine = ---;

Engine :: struct {
    scenes:           []Scene;
    should_terminate: bool;
}

Engine_Config :: struct {
    name:             string;
    target_fps:       u32 = 60;
    window_size:      struct {
        width:        u32 = 800;
        height:       u32 = 600;
    };
    scenes:           []Scene;
    initial_scene_id: u32;
    clear_color:      Vector4;
}

make_view_matrix :: (transform: Transform_Component, camera: Camera_Component) -> Matrix4 {
    position := Vector3.{
        transform.position.x * PIXELS_PER_UNIT,
        transform.position.y * PIXELS_PER_UNIT,
        0,
    };

    return make_look_at_matrix(position, position + camera.front, camera.up, false);
}

FPS_Manager:    : #import "FPS_Manager";
Window_Manager: : #import "Window_Manager";
#load "systems/physics_system.jai";
