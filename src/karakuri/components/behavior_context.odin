package components

import "../../kec"
import "../../kwindow/input_manager"
import "../timer"

Behavior_Context :: struct {
	entity:   kec.Entity,
	dt:       f64,
	spawner:  ^Spawner_Info,
	registry: kec.Registry,
	input:    struct {
		is_key_pressed:  input_manager.Key_State_Proc,
		is_key_down:     input_manager.Key_State_Proc,
		is_key_up:       input_manager.Key_State_Proc,
		is_key_released: input_manager.Key_State_Proc,
	},
	timer:    struct {
		timer_info:     ^timer.TimerInfo,
		set_timeout:    type_of(timer.set_timeout),
		set_interval:   type_of(timer.set_interval),
		clear_timeout:  type_of(timer.clear_timeout),
		clear_interval: type_of(timer.clear_interval),
	},
}

