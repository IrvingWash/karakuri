package shmup

import "core:log"
import "karakuri:karakuri/components"
import "karakuri:kmath"
import "karakuri:kutils"

Projectile_Shooter :: enum {
	Enemy,
	Player,
}

projectile_prefab :: proc(
	start_position: kmath.Vector2,
	owner: Projectile_Shooter,
) -> components.Component_Bundle {
	velocity_multiplier: f64 = owner == .Player ? -1 : 1
	tag := owner == .Player ? "player_projectile" : "enemy_projectile"

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
			velocity = kmath.Vector2{0, 700 * velocity_multiplier},
		},
		tag = components.Tag_Component{value = tag},
	}
}

@(private = "file")
on_start: components.Lifecycle_Proc : proc(ctx: components.Behavior_Context) {
	log.info("Projectile started")
}

@(private = "file")
on_destroy: components.Lifecycle_Proc : proc(
	ctx: components.Behavior_Context,
) {
	log.info("Projectile destroyed")
}

