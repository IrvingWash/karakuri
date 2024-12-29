package kwindow

import "core:strings"
import rl "vendor:raylib"

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

	rl.InitWindow(i32(width), i32(height), strings.clone_to_cstring(title))

	if fullscreen && !rl.IsWindowFullscreen() {
		rl.ToggleFullscreen()
	}
}

destroy_window :: proc() {
	rl.CloseWindow()
}
