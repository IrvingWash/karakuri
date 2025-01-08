package karakuri_world

import "ktimer:timer"

// Lifecycle behavior of an entity
Behavior :: struct {
	on_start:   Maybe(Lifecycle_Proc),
	on_update:  Maybe(Lifecycle_Proc),
	on_destroy: Maybe(Lifecycle_Proc),
	on_timer:   Maybe(On_Timer_Proc),
}

// A Procedure which is called on every lifecycle event
Lifecycle_Proc :: proc(ctx: Behavior_Context)

// A procedure which is called on every timer event
On_Timer_Proc :: proc(
	ctx: Behavior_Context,
	finished_timers: map[uint]struct {},
)

// Context passed to lifecycle procedures
Behavior_Context :: struct {
	self:       ^Entity,
	delta_time: f64,
	world:      ^World,
	timer_info: ^timer.Timer_Info,
}

