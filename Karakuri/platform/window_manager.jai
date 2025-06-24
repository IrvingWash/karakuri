WC :: #import "Window_Creation";

init_window_manager :: (width: u32, height: u32, title: string) -> WC.Window_Type {
    manager = .{
        window = WC.create_window(width, height, title),
    };

    return manager.window;
}

toggle_fullscreen :: () {
    WC.toggle_fullscreen(manager.window, true, null);
}

#scope_file

Window_Manager :: struct {
    window: WC.Window_Type;
}

manager: Window_Manager = ---;
