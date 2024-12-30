package components

import "../../kec"

Behavior_Component :: struct {
	on_start:	Maybe(On_Start_Fn),
	on_update:	Maybe(On_Update_Fn),
	on_destroy: Maybe(On_Destroy_Fn),
}

On_Start_Fn :: proc(ctx: Behavior_Context)
On_Update_Fn :: proc(ctx: Behavior_Context)
On_Destroy_Fn :: proc(ctx: Behavior_Context)

Behavior_Context :: struct {
	entity: kec.Entity,
	dt: f64,
}
