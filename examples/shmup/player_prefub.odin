package shmup

import "core:fmt"
import "karakuri:karakuri/components"
import "karakuri:kec"
import "karakuri:kutils"

player_prefub :: proc() -> components.Component_Bundle {
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
		},
	}
}

SPEED :: 400

move :: proc(ctx: components.Behavior_Context) {
	transform := kec.get_component(
		ctx.registry,
		ctx.entity,
		components.Transform_Component,
	)

	velocity := [2]f64{}

	if ctx.input.is_key_down(kutils.Key.W) {
		velocity.y = -SPEED * ctx.dt
	}
	if ctx.input.is_key_down(kutils.Key.S) {
		velocity.y = SPEED * ctx.dt
	}
	if ctx.input.is_key_down(kutils.Key.A) {
		velocity.x = -SPEED * ctx.dt
	}
	if ctx.input.is_key_down(kutils.Key.D) {
		velocity.x = SPEED * ctx.dt
	}

	transform.position += velocity
}

on_update: components.On_Update_Proc : proc(ctx: components.Behavior_Context) {
	move(ctx)
}

on_start: components.On_Start_Proc : proc(ctx: components.Behavior_Context) {
	fmt.println("Player started")
}

on_destroy: components.On_Destroy_Proc : proc(
	ctx: components.Behavior_Context,
) {
	fmt.println("Player destroyed")
}
