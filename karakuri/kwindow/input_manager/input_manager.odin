package input_manager

import rl "vendor:raylib"

Key :: rl.KeyboardKey

is_key_pressed :: proc(key: Key) -> bool {
	return rl.IsKeyPressed(key)
}

is_key_down :: proc(key: Key) -> bool {
	return rl.IsKeyDown(key)
}

is_key_up :: proc(key: Key) -> bool {
	return rl.IsKeyUp(key)
}

is_key_released :: proc(key: Key) -> bool {
	return rl.IsKeyReleased(key)
}

// TODO: private
is_quit_requested :: proc() -> bool {
	return rl.WindowShouldClose()
}
