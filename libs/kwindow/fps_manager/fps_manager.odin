package kwindow_fps_manager

import rl "vendor:raylib"

// Sets target FPS
set_target_fps :: proc(fps: uint) {
	rl.SetTargetFPS(i32(fps))
}

// Returns delta time in seconds
get_delta_time :: proc() -> f64 {
	return f64(rl.GetFrameTime())
}

// Returns elapsed time in milliseconds
get_time :: proc() -> f64 {
	return rl.GetTime() * 1000
}

