package components

import "../../kec"

Behavior_Component :: struct {
	on_start:     Maybe(Lifecycle_Proc),
	on_update:    Maybe(Lifecycle_Proc),
	on_destroy:   Maybe(Lifecycle_Proc),
	on_collision: Maybe(On_Collision_Proc),
}

Lifecycle_Proc :: proc(ctx: Behavior_Context)
On_Collision_Proc :: proc(ctx: Behavior_Context, other: kec.Entity)
