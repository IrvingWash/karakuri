package shmup

import "core:log"
import "karakuri:karakuri/components"
import "karakuri:kec"
import "karakuri:kmath"
import "karakuri:kutils"

enemy_prefab :: proc(position: kmath.Vector2) -> components.Component_Bundle {
	return components.Component_Bundle {
		tag = components.Tag_Component{value = "enemy"},
		transform = components.new_transform_component(position = position),
		rigid_body = components.Rigid_Body_Component {
			velocity = kmath.Vector2{0, 50},
		},
		shape = components.Shape_Component {
			size = kmath.Vector2{30, 50},
			color = kutils.ColorBlue,
		},
		behavior = components.Behavior_Component {
			on_collision = on_collision,
			on_start = on_start,
			on_destroy = on_destroy,
			on_timer = on_timer,
		},
	}
}

shoot_interval_id: uint

@(private = "file")
on_start: components.Lifecycle_Proc : proc(ctx: components.Behavior_Context) {
	log.info("Enemy started")

	shoot_interval_id = ctx.timer.set_interval(ctx.timer.timer_info, 1000)
}

@(private = "file")
on_destroy: components.Lifecycle_Proc : proc(
	ctx: components.Behavior_Context,
) {
	log.info("Enemy destroyed")

	ctx.timer.clear_interval(ctx.timer.timer_info, shoot_interval_id)
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
	); other_tag != nil && other_tag.value == "player_projectile" {
		ctx.spawner.remove_entity(ctx.spawner, ctx.entity)
	}
}

@(private = "file")
on_timer: components.On_Timer_Proc : proc(
	ctx: components.Behavior_Context,
	finished_timers: map[uint]struct {},
) {
	if shoot_interval_id in finished_timers {
		transform := kec.get_component(
			ctx.registry,
			ctx.entity,
			components.Transform_Component,
		)

		ctx.spawner.add_entity(
			ctx.spawner,
			projectile_prefab(transform.position, .Enemy),
		)
	}
}

