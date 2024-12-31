package input_manager

import rl "vendor:raylib"

// List of keyboard keys that can be tested
Key :: rl.KeyboardKey

// Proc type for key state checkers
Key_State_Proc :: proc(key: Key) -> bool

// Checks for a pressed key.
// If the key is held down, this proc returns true only once
is_key_pressed: Key_State_Proc : proc(key: Key) -> bool {
	return rl.IsKeyPressed(key)
}

// Checks for a held down key.
// Returns true until the key is not released
is_key_down: Key_State_Proc : proc(key: Key) -> bool {
	return rl.IsKeyDown(key)
}

// Checks for a key that is not pressed
// Returns true until the keys is pressed
is_key_up: Key_State_Proc : proc(key: Key) -> bool {
	return rl.IsKeyUp(key)
}

// Checks for a key that just has been released
// Returns true only if the previous state for this key was pressed/down
is_key_released: Key_State_Proc : proc(key: Key) -> bool {
	return rl.IsKeyReleased(key)
}

// Checks whether a window close request was received
is_quit_requested :: proc() -> bool {
	return rl.WindowShouldClose()
}
