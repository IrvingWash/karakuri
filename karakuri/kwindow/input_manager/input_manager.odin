package input_manager

import rl "vendor:raylib"

Key :: rl.KeyboardKey

Key_State_Fn :: proc(key: Key) -> bool

is_key_pressed: Key_State_Fn : proc(key: Key) -> bool {
	return rl.IsKeyPressed(key)
}

is_key_down: Key_State_Fn : proc(key: Key) -> bool {
	return rl.IsKeyDown(key)
}

is_key_up: Key_State_Fn : proc(key: Key) -> bool {
	return rl.IsKeyUp(key)
}

is_key_released: Key_State_Fn : proc(key: Key) -> bool {
	return rl.IsKeyReleased(key)
}

// TODO: private
is_quit_requested :: proc() -> bool {
	return rl.WindowShouldClose()
}
