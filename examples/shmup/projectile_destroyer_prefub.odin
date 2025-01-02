package shmup

import "karakuri:karakuri/components"
import "karakuri:kec"
import "karakuri:kmath"
import "karakuri:kutils"

projectile_destroyer_prefub :: proc(
	position: kmath.Vector2,
) -> components.Component_Bundle {
	return components.Component_Bundle {
		transform = components.new_transform_component(position = position),
		shape = components.Shape_Component {
			color = kutils.ColorGreen,
			size = {DODONPACHI_WIDTH * 2, 10},
		},
		behavior = components.Behavior_Component{on_collision = on_collision},
	}
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
	if other_tag != nil && other_tag.value == "projectile" {
		ctx.spawner.remove_entity(ctx.spawner, other)
	}
}
