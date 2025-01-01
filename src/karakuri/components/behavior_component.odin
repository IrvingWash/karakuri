package components

Behavior_Component :: struct {
	on_start:   On_Start_Proc,
	on_update:  On_Update_Proc,
	on_destroy: On_Destroy_Proc,
}

On_Start_Proc :: proc()
On_Update_Proc :: proc()
On_Destroy_Proc :: proc()
