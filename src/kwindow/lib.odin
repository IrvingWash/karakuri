package kwindow

import "core:strings"
import rl "vendor:raylib"

// Opens a window and initializes rendering context
create_window :: proc(
	title: string,
	width: uint,
	height: uint,
	fullscreen := true,
	vsync := true,
) {
	if vsync {
		rl.SetConfigFlags(rl.ConfigFlags{rl.ConfigFlag.VSYNC_HINT})
	}

	title_raw := strings.clone_to_cstring(title)
	defer delete(title_raw)

	rl.SetTraceLogLevel(rl.TraceLogLevel.WARNING)

	rl.InitWindow(i32(width), i32(height), title_raw)

	if fullscreen && !rl.IsWindowFullscreen() {
		rl.ToggleFullscreen()
	}
}

// Closes the opened window and cleans up the rendering context
destroy_window :: proc() {
	rl.CloseWindow()
}

