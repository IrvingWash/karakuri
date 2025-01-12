package karakuri_input_manager

import "kwindow:input_manager"
import "kutils:keyboard"
import v2 "kmath:vector2"

// Checks if the keys is down since the current frame
is_key_pressed :: proc(key: keyboard.Key) -> bool {
	return input_manager.is_key_pressed(key)
}

// Checks if the key is down regardles of the previous state
is_key_down :: proc(key: keyboard.Key) -> bool {
	return input_manager.is_key_down(key)
}

// Checks if the key is not down/pressed
is_key_up :: proc(key: keyboard.Key) -> bool {
	return input_manager.is_key_up(key)
}

// Returns the cursor position
get_cursor_position :: proc() -> v2.Vector2 {
	return input_manager.get_cursor_position()
}

// Checks if the key was released since the previous frame
is_key_released :: proc(key: keyboard.Key) -> bool {
	return input_manager.is_key_released(key)
}

