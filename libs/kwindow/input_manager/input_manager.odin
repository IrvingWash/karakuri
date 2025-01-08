package input_manager

import rl "vendor:raylib"
import "kutils/keyboard"

// Checks if the keys is down since the current frame
is_key_pressed :: proc(key: kutils.Key) -> bool {
	return rl.IsKeyPressed(key)
}

// Checks if the key is down regardles of the previous state
is_key_down :: proc(key: kutils.Key) -> bool {
	return rl.IsKeyDown(key)
}

// Checks if the key is not down/pressed
is_key_up :: proc(key: kutils.Key) -> bool {
	return rl.IsKeyUp(key)
}

// Checks if the key was released since the previous frame
is_key_released :: proc(key: kutils.Key) -> bool {
	return rl.IsKeyReleased(key)
}

// Checks if the quit command was received
is_quit_requested :: proc() -> bool {
	return rl.WindowShouldClose()
}

