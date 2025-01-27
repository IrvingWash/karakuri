package example_shmup_prefabs

import "core:strings"
import "core:log"
import v2 "kmath:vector2"
import "karakuri:world"
import "karakuri:components"

bullet_destroyer_prefab :: proc(position: v2.Vector2) -> world.Entity_Payload {
	behavior := new(Bullet_Destroyer)
	behavior^ = Bullet_Destroyer {
		on_start     = on_start,
		on_destroy   = on_destroy,
		on_collision = on_collision,
	}

	return world.Entity_Payload {
		tag = "Bullet Destroyer",
		transform = components.Transform_Component{position = position},
		box_collider = components.Box_Collider_Component{size = {1000, 10}},
		behavior = behavior,
	}
}

Bullet_Destroyer :: struct {
	using behavior: world.Behavior,
}

@(private = "file")
on_start: world.Lifecycle_Proc : proc(ctx: world.Behavior_Context) {
	log.info("Projectile Destroyer started")
}

@(private = "file")
on_destroy: world.Lifecycle_Proc : proc(ctx: world.Behavior_Context) {
	log.info("Projectile Destroyer destroyed")
}

@(private = "file")
on_collision: world.On_Collision_Proc : proc(
	ctx: world.Behavior_Context,
	other: ^world.Entity,
) {
	tag, ok := &other.tag.?
	if !ok || !strings.contains(tag^, "Bullet") {
		return
	}

	world.remove_entity(ctx.world, other.token)
}

