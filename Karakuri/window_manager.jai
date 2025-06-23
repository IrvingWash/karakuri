#scope_module;

WC :: #import "Window_Creation";
Simp :: #import "Simp";
Input :: #import "Input";

Window_Manager :: struct {
    window: WC.Window_Type;
    width: u32;
    height: u32;
    events: Events;
}

Events :: struct {
    quit: bool;
}

init_window_manager :: (width: u32, height: u32, title: string) -> Window_Manager {
    manager: Window_Manager;

    manager.window = WC.create_window(width, height, title);
    manager.width = width;
    manager.height = height;

    Simp.set_render_target(manager.window);

    return manager;
}

get_events :: (manager: *Window_Manager) {
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
}

get_window_dimensions :: (manager: Window_Manager) -> width: u32, height: u32 {
    return manager.width, manager.height;
}

swap_buffers :: (manager: Window_Manager) {
    Simp.swap_buffers(manager.window);
}

toggle_fullscreen :: (manager: Window_Manager) {
    WC.toggle_fullscreen(manager.window, true, null);
}
