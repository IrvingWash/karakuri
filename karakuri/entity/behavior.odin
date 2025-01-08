package karakuri_entity

// Lifecycle behavior of an entity
Behavior :: struct {
	on_start:   Maybe(Lifecycle_Proc),
	on_update:  Maybe(Lifecycle_Proc),
	on_destroy: Maybe(Lifecycle_Proc),
}

Lifecycle_Proc :: proc()

