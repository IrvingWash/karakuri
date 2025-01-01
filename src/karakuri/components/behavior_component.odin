package components

Behavior_Component :: struct {
	on_start:   Maybe(On_Start_Proc),
	on_update:  Maybe(On_Update_Proc),
	on_destroy: Maybe(On_Destroy_Proc),
}

On_Start_Proc :: proc(ctx: Behavior_Context)
On_Update_Proc :: proc(ctx: Behavior_Context)
On_Destroy_Proc :: proc(ctx: Behavior_Context)
