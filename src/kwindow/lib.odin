package kwindow

import "core:strings"
import rl "vendor:raylib"

// create_window opens a window with the specified title, dimensions, and optional fullscreen/VSync flags.
// It allocates a temporary C-string for Raylib's InitWindow, freeing it before the function returns.
create_window :: proc(
    title: string,
    width: u32,
    height: u32,
    fullscreen: bool = true,
    vsync: bool = true,
) {
    if vsync {
        rl.SetConfigFlags(rl.ConfigFlags{
            rl.ConfigFlag.VSYNC_HINT,
        })
    }

    // Convert the given Odin string to a temporary C string for Raylib.
    // Freed automatically at the end of this procedure (due to 'defer').
    title_raw := strings.clone_to_cstring(title)
    defer strings.delete(title_raw)

    rl.InitWindow(i32(width), i32(height), title_raw)

    if fullscreen and not rl.IsWindowFullscreen() {
        rl.ToggleFullscreen()
    }
}

// destroy_window closes the opened Raylib window and cleans up its associated resources.
destroy_window :: proc() {
    rl.CloseWindow()
}
