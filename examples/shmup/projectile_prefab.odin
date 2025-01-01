package shmup

import "core:fmt"
import "karakuri:karakuri/components"
import "karakuri:kec"
import "karakuri:kmath"
import "karakuri:kutils"

projectile_prefab :: proc(
	start_position: kmath.Vector2,
) -> components.Component_Bundle {
	return components.Component_Bundle {
		transform = components.new_transform_component(
			position = start_position,
		),
		shape = components.Shape_Component {
			color = kutils.ColorRed,
			size = kmath.Vector2{10, 10},
		},
		behavior = components.Behavior_Component {
			on_start = on_start,
			on_update = on_update,
			on_destroy = on_destroy,
		},
	}
}

PROJECTILE_SPEED :: 700

@(private = "file")
on_update: components.On_Update_Proc : proc(ctx: components.Behavior_Context) {
	transform := kec.get_component(
		ctx.registry,
		ctx.entity,
		components.Transform_Component,
	)

	transform.position += kmath.Vector2{0, -PROJECTILE_SPEED * ctx.dt}
}

@(private = "file")
on_start: components.On_Start_Proc : proc(ctx: components.Behavior_Context) {
	fmt.println("Projectile started")
}

@(private = "file")
on_destroy: components.On_Destroy_Proc : proc(
	ctx: components.Behavior_Context,
) {
	fmt.println("Projectile destroyed")
}
