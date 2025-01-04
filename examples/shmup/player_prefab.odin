package shmup

import "core:log"
import "karakuri:karakuri/components"
import "karakuri:kec"
import "karakuri:kutils"

player_prefab :: proc() -> components.Component_Bundle {
	return components.Component_Bundle {
		transform = components.new_transform_component(position = {0, 200}),
		shape = components.Shape_Component {
			color = kutils.ColorYellow,
			size = {30, 50},
		},
		behavior = components.Behavior_Component {
			on_start = on_start,
			on_update = on_update,
			on_destroy = on_destroy,
			on_collision = on_collision,
		},
	}
}

PLAYER_SPEED :: 400

player_shoot :: proc(ctx: components.Behavior_Context) {
	if ctx.input.is_key_pressed(kutils.Key.SPACE) {
		position :=
			kec.get_component(ctx.registry, ctx.entity, components.Transform_Component).position

		ctx.spawner.add_entity(
			ctx.spawner,
			projectile_prefab(position, .Player),
		)
	}
}

player_move :: proc(ctx: components.Behavior_Context) {
	transform := kec.get_component(
		ctx.registry,
		ctx.entity,
		components.Transform_Component,
	)

	velocity := [2]f64{}

	if ctx.input.is_key_down(kutils.Key.W) {
		velocity.y = -PLAYER_SPEED * ctx.dt
	}
	if ctx.input.is_key_down(kutils.Key.S) {
		velocity.y = PLAYER_SPEED * ctx.dt
	}
	if ctx.input.is_key_down(kutils.Key.A) {
		velocity.x = -PLAYER_SPEED * ctx.dt
	}
	if ctx.input.is_key_down(kutils.Key.D) {
		velocity.x = PLAYER_SPEED * ctx.dt
	}

	transform.position += velocity
}

@(private = "file")
on_update: components.Lifecycle_Proc : proc(ctx: components.Behavior_Context) {
	player_shoot(ctx)
	player_move(ctx)
}

@(private = "file")
on_start: components.Lifecycle_Proc : proc(ctx: components.Behavior_Context) {
	log.info("Player started")
}

@(private = "file")
on_destroy: components.Lifecycle_Proc : proc(
	ctx: components.Behavior_Context,
) {
	log.info("Player destroyed")
}

@(private = "file")
on_collision: components.On_Collision_Proc : proc(
	ctx: components.Behavior_Context,
	other: kec.Entity,
) {
	if tag := kec.get_component(ctx.registry, other, components.Tag_Component);
	   tag != nil && tag.value == "enemy_projectile" {
		ctx.spawner.remove_entity(ctx.spawner, ctx.entity)
	}
}

