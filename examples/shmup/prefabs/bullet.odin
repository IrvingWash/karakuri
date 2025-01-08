package example_shmup_prefabs

import "core:fmt"
import v2 "kmath:vector2"
import "kutils:color"
import "karakuri:world"
import "karakuri:components"

bullet_prefab :: proc(position: v2.Vector2) -> world.Entity_Payload {
	bullet_behavior := new(Bullet_Behavior)
	bullet_behavior^ = Bullet_Behavior {
		speed      = 700,
		on_start   = on_start,
		on_destroy = on_destroy,
	}

	return world.Entity_Payload {
		tag = "buulet",
		transform = components.Transform_Component {
			position = position,
			scale = v2.Unit,
		},
		shape = components.Shape_Component {
			size = v2.Vector2{10, 10},
			color = color.Red,
		},
	}
}

@(private = "file")
Bullet_Behavior :: struct {
	using behavior: world.Behavior,
	speed:          f64,
}

@(private = "file")
on_start: world.Lifecycle_Proc : proc(ctx: world.Behavior_Context) {
	fmt.println("Bullet started")
}

@(private = "file")
on_destroy: world.Lifecycle_Proc : proc(ctx: world.Behavior_Context) {
	fmt.println("Bullet destroyed")
}

@(private = "file")
on_update: world.Lifecycle_Proc : proc(ctx: world.Behavior_Context) {
	transform := &ctx.self.transform
	behavior := world.get_behavior(ctx.self^, Bullet_Behavior).?

	move(transform, behavior, ctx.delta_time)
}

@(private = "file")
move :: proc(
	transform: ^components.Transform_Component,
	behavior: ^Bullet_Behavior,
	dt: f64,
) {
	transform.position.y -= behavior.speed * dt
}

