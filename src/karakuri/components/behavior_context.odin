package components

import "../../kec"
import "../../kwindow/input_manager"

Behavior_Context :: struct {
	entity:  kec.Entity,
	dt:      f64,
	spawner: ^Spawner_Info,
	input:   struct {
		is_key_pressed:  input_manager.Key_State_Proc,
		is_key_down:     input_manager.Key_State_Proc,
		is_key_up:       input_manager.Key_State_Proc,
		is_key_released: input_manager.Key_State_Proc,
	},
}
