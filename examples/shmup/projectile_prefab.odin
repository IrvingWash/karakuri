package shmup

import "core:fmt"
import "karakuri:karakuri/components"
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
			on_destroy = on_destroy,
		},
		rigid_body = components.Rigid_Body_Component {
			velocity = kmath.Vector2{0, -700},
		},
		tag = components.Tag_Component{value = "projectile"},
	}
}

@(private = "file")
on_start: components.Lifecycle_Proc : proc(ctx: components.Behavior_Context) {
	fmt.println("Projectile started")
}

@(private = "file")
on_destroy: components.Lifecycle_Proc : proc(
	ctx: components.Behavior_Context,
) {
	fmt.println("Projectile destroyed")
}
