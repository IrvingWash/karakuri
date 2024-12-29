package components

Behavior_Component :: struct {
	on_start:   Maybe(On_Start_Fn),
	on_update:  Maybe(On_Update_Fn),
	on_destroy: Maybe(On_Destroy_Fn),
}

On_Start_Fn :: proc()
On_Update_Fn :: proc()
On_Destroy_Fn :: proc()
