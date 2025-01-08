package kwindow_window_creation

import "core:strings"
import rl "vendor:raylib"

// Initialzes and opens a window
create_window :: proc(
	title: string,
	width, height: uint,
	fullscreen, resizable: bool,
	vsync: bool,
) {
	if resizable {
		rl.SetConfigFlags(rl.ConfigFlags{.WINDOW_RESIZABLE})
	}

	if vsync {
		rl.SetConfigFlags(rl.ConfigFlags{.VSYNC_HINT})
	}

	rl.SetTraceLogLevel(.WARNING)

	title_raw := strings.clone_to_cstring(title)
	defer delete(title_raw)

	rl.InitWindow(i32(width), i32(height), title_raw)

	if fullscreen && !rl.IsWindowFullscreen() {
		rl.ToggleFullscreen()
	}
}

// Destroys the opened window
destroy_window :: proc() {
	rl.CloseWindow()
}

