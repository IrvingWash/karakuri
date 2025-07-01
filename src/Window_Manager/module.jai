// A simple window manager. Maybe this should be replaced with GLFW

WC :: #import "Window_Creation";
#import "Window_Type";

// Initializes the manager, creates a window and returns a handle to it
init :: (width: u32, height: u32, title: string) -> ok: bool, Window_Type {
    window := WC.create_window(width, height, title);
    if !window {
        return false, INVALID_WINDOW;
    }

    manager = .{
        window = window,
    };

    return true, manager.window;
}

#scope_file

Window_Manager :: struct {
    window: WC.Window_Type;
}

manager: Window_Manager = ---;
