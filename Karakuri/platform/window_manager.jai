WC :: #import "Window_Creation";
Simp :: #import "Simp";
Input :: #import "Input";

Events :: struct {
    quit: bool;
}

init_window_manager :: (width: u32, height: u32, title: string) {
    manager = .{
        window = WC.create_window(width, height, title),
        width = width,
        height = height,
    };

    Simp.set_render_target(manager.window);
}

get_events :: () -> Events {
    Input.update_window_events();

    for Input.events_this_frame {
        if it.type == {
            case .QUIT;
                manager.events.quit = true;
        }
    }

    for Input.get_window_resizes() {
        Simp.update_window(it.window);

        if it.window == manager.window {
            manager.width = xx it.width;
            manager.height = xx it.height;
        }
    }

    return manager.events;
}

get_window_dimensions :: () -> width: u32, height: u32 {
    return manager.width, manager.height;
}

swap_buffers :: () {
    Simp.swap_buffers(manager.window);
}

toggle_fullscreen :: () {
    WC.toggle_fullscreen(manager.window, true, null);
}

#scope_file

Window_Manager :: struct {
    window: WC.Window_Type;
    width: u32;
    height: u32;
    events: Events;
}

manager: Window_Manager = ---;
