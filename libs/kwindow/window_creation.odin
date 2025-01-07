package kwindow

import "core:strings"
import rl "vendor:raylib"

create_window :: proc(
	title: string,
	width: uint,
	height: uint,
	fullscreen: bool,
	resizable: bool,
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

	rl.InitWindow(i32(width), i32(height))

	if fullscreen && !rl.IsWindowFullscreen() {
		rl.ToggleFullscreen()
	}
}

destroy_window :: proc() {
	rl.CloseWindow()
}

