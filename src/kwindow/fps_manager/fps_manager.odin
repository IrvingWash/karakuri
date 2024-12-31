package fps_manager

import rl "vendor:raylib"

// Sets the desired FPS
set_target_fps :: proc(fps: uint) {
	rl.SetTargetFPS(i32(fps))
}

// Returns the time difference in milliseconds between the current and previous frame.
get_delta_time :: proc() -> f64 {
	return f64(rl.GetFrameTime())
}

// Returns the time in seconds since window was opened.
get_time :: proc() -> f64 {
	return rl.GetTime()
}
