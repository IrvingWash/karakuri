package shmup

import "core:log"
import "core:strings"
import "karakuri:karakuri/components"
import "karakuri:kec"
import "karakuri:kmath"
import "karakuri:kutils"

projectile_destroyer_prefab :: proc(
	position: kmath.Vector2,
) -> components.Component_Bundle {
	return components.Component_Bundle {
		transform = components.new_transform_component(position = position),
		shape = components.Shape_Component {
			color = kutils.ColorGreen,
			size = {DODONPACHI_WIDTH * 2, 10},
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
	log.info("Projectile Destroyer started")
}

@(private = "file")
on_destroy: components.Lifecycle_Proc : proc(
	ctx: components.Behavior_Context,
) {
	log.info("Projectile Destroyer destroyed")
}

@(private = "file")
on_collision: components.On_Collision_Proc : proc(
	ctx: components.Behavior_Context,
	other: kec.Entity,
) {
	other_tag := kec.get_component(
		ctx.registry,
		other,
		components.Tag_Component,
	)
	if other_tag != nil && strings.contains(other_tag.value, "projectile") {
		ctx.spawner.remove_entity(ctx.spawner, other)
	}
}

