package shmup

import "core:fmt"
import "karakuri:karakuri/components"
import "karakuri:kec"
import "karakuri:kmath"
import "karakuri:kutils"

enemy_prefab :: proc(position: kmath.Vector2) -> components.Component_Bundle {
	return components.Component_Bundle {
		tag = components.Tag_Component{value = "enemy"},
		transform = components.new_transform_component(position = position),
		shape = components.Shape_Component {
			size = kmath.Vector2{30, 50},
			color = kutils.ColorBlue,
		},
		behavior = components.Behavior_Component {
			on_collision = on_collision,
			on_start = on_start,
			on_destroy = on_destroy,
		},
	}
}

@(private = "file")
on_start: components.Lifecycle_Proc : proc(ctx: components.Behavior_Context) {
	fmt.println("Enemy started")
}

@(private = "file")
on_destroy: components.Lifecycle_Proc : proc(
	ctx: components.Behavior_Context,
) {
	fmt.println("Enemy destroyed")
}

@(private = "file")
on_collision: components.On_Collision_Proc : proc(
	ctx: components.Behavior_Context,
	other: kec.Entity,
) {
	if other_tag := kec.get_component(
		ctx.registry,
		other,
		components.Tag_Component,
	); other_tag != nil {
		ctx.spawner.remove_entity(ctx.spawner, ctx.entity)
	}
}

