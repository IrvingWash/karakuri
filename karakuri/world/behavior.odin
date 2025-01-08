package karakuri_world

import "ktimer:timer"

// Lifecycle behavior of an entity
Behavior :: struct {
	on_start:   Maybe(Lifecycle_Proc),
	on_update:  Maybe(Lifecycle_Proc),
	on_destroy: Maybe(Lifecycle_Proc),
}

// Lifecycle procedure type that should be implemented
Lifecycle_Proc :: proc(ctx: Behavior_Context)

// Context passed to lifecycle procedures
Behavior_Context :: struct {
	self:       ^Entity,
	delta_time: f64,
	world:      ^World,
	timer:      ^timer.Timer_Info,
}

