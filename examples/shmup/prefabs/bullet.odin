package example_shmup_prefabs

import "core:log"
import "core:strings"
import v2 "kmath:vector2"
import "kutils:color"
import "karakuri:world"
import "karakuri:components"

bullet_prefab :: proc(
	position: v2.Vector2,
	speed: f64,
	tag: string,
) -> world.Entity_Payload {
	bullet_behavior := new(Bullet_Behavior)
	bullet_behavior^ = Bullet_Behavior {
		speed        = speed,
		on_start     = on_start,
		on_update    = on_update,
		on_destroy   = on_destroy,
		on_collision = on_collision,
	}

	return world.Entity_Payload {
		tag = tag,
		transform = components.Transform_Component{position = position},
		shape = components.Shape_Component {
			size = v2.Vector2{10, 10},
			color = color.Transparent,
		},
		box_collider = components.Box_Collider_Component{},
		behavior = bullet_behavior,
		sprite = components.Sprite_Component {
			sprite_name = speed < 0 ? "bullet_blue" : "bullet_green",
			flip = {y = speed < 0 ? true : false},
			sorting_layer = 2,
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
	log.info("Bullet started ", ctx.self.tag)
}

@(private = "file")
on_destroy: world.Lifecycle_Proc : proc(ctx: world.Behavior_Context) {
	log.info("Bullet destroyed")
}

@(private = "file")
on_update: world.Lifecycle_Proc : proc(ctx: world.Behavior_Context) {
	transform := &ctx.self.transform
	behavior := world.get_behavior(ctx.self^, Bullet_Behavior).?

	move(transform, behavior, ctx.delta_time)
}

@(private = "file")
on_collision: world.On_Collision_Proc : proc(
	ctx: world.Behavior_Context,
	other: ^world.Entity,
) {
	tag := ctx.self.tag.?

	if other.tag == "Enemy" && !strings.contains(tag, "Enemy") {
		world.remove_entity(ctx.world, ctx.self.token)
	}
}

@(private = "file")
move :: proc(
	transform: ^components.Transform_Component,
	behavior: ^Bullet_Behavior,
	dt: f64,
) {
	transform.position.y -= behavior.speed * dt
}

