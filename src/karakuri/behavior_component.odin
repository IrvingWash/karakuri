package karakuri

import "../kec"
import input_manager "../kwindow/input_manager"

Behavior_Component :: struct {
	on_start:   Maybe(On_Start_Proc),
	on_update:  Maybe(On_Update_Proc),
	on_destroy: Maybe(On_Destroy_Proc),
}

On_Start_Proc :: proc(ctx: Behavior_Context)
On_Update_Proc :: proc(ctx: Behavior_Context)
On_Destroy_Proc :: proc(ctx: Behavior_Context)

Behavior_Context :: struct {
	entity: kec.Entity,
	dt:     f64,
	input:  struct {
		is_key_pressed:  input_manager.Key_State_Proc,
		is_key_down:     input_manager.Key_State_Proc,
		is_key_up:       input_manager.Key_State_Proc,
		is_key_released: input_manager.Key_State_Proc,
	},
}
