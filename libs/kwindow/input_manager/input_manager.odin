package kwindow_input_manager

import rl "vendor:raylib"
import "kutils:keyboard"
import v2 "kmath:vector2"

// Checks if the keys is down since the current frame
is_key_pressed :: proc(key: keyboard.Key) -> bool {
	return rl.IsKeyPressed(key)
}

// Checks if the key is down regardles of the previous state
is_key_down :: proc(key: keyboard.Key) -> bool {
	return rl.IsKeyDown(key)
}

// Checks if the key is not down/pressed
is_key_up :: proc(key: keyboard.Key) -> bool {
	return rl.IsKeyUp(key)
}

// Checks if the key was released since the previous frame
is_key_released :: proc(key: keyboard.Key) -> bool {
	return rl.IsKeyReleased(key)
}

// Returns the cursor coordinates
get_cursor_position :: proc() -> v2.Vector2 {
	mouse_position := rl.GetMousePosition()

	return v2.Vector2 {
		f64(mouse_position.x) - f64(rl.GetScreenWidth() / 2),
		f64(mouse_position.y) - f64(rl.GetScreenHeight() / 2),
	}
}

// Checks if the quit command was received
is_quit_requested :: proc() -> bool {
	return rl.WindowShouldClose()
}

