package components

import kec "../../kec"
import im "../../kwindow/input_manager"

Behavior_Component :: struct {
	on_start:   Maybe(On_Start_Fn),
	on_update:  Maybe(On_Update_Fn),
	on_destroy: Maybe(On_Destroy_Fn),
}

On_Start_Fn :: proc(ctx: Behavior_Context)
On_Update_Fn :: proc(ctx: Behavior_Context)
On_Destroy_Fn :: proc(ctx: Behavior_Context)

Behavior_Context :: struct {
	entity:          kec.Entity,
	registry:        kec.Registry,
	dt:              f64,
	is_key_pressed:  im.Key_State_Fn,
	is_key_up:       im.Key_State_Fn,
	is_key_down:     im.Key_State_Fn,
	is_key_released: im.Key_State_Fn,
}
