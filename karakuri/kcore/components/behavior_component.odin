package components

On_Start :: proc()
On_Update :: proc()
On_Destroy :: proc()

Behavior_Component :: struct {
	on_start:   Maybe(On_Start),
	on_update:  Maybe(On_Update),
	on_destroy: Maybe(On_Destroy),
}
