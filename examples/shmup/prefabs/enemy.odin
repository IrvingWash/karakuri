package example_shmup_prefabs

import "core:log"
import v2 "kmath:vector2"
import "ktimer:timer"
import "karakuri:world"
import "karakuri:components"

enemy_prefab :: proc(position: v2.Vector2) -> world.Entity_Payload {
	enemy_behavior := new(Enemy_Behavior)
	enemy_behavior^ = Enemy_Behavior {
		speed        = 50,
		on_start     = on_start,
		on_destroy   = on_destroy,
		on_update    = on_update,
		on_timer     = on_timer,
		on_collision = on_collision,
	}

	return world.Entity_Payload {
		tag = "Enemy",
		transform = components.Transform_Component{position = position},
		box_collider = components.Box_Collider_Component{size = {45, 30}},
		behavior = enemy_behavior,
		sprite = components.Sprite_Component{
 			sprite_name = "enemy_straight",
 			sorting_layer = 3,
 		},
	}
}

@(private = "file")
Enemy_Behavior :: struct {
	using behavior: world.Behavior,
	speed:          f64,
	shot_timer_id:  uint,
}

@(private = "file")
on_start: world.Lifecycle_Proc : proc(ctx: world.Behavior_Context) {
	log.info("Enemy started")

	behavior := world.get_behavior(ctx.self^, Enemy_Behavior).?

	behavior.shot_timer_id = timer.set_interval(ctx.timer_info, 1000)
}

@(private = "file")
on_destroy: world.Lifecycle_Proc : proc(ctx: world.Behavior_Context) {
	log.info("Enemy destroyed")

	behavior := world.get_behavior(ctx.self^, Enemy_Behavior).?

	timer.clear_interval(ctx.timer_info, behavior.shot_timer_id)
}

@(private = "file")
on_timer: world.On_Timer_Proc : proc(
	ctx: world.Behavior_Context,
	finished_timers: map[uint]struct {},
) {
	behavior := world.get_behavior(ctx.self^, Enemy_Behavior).?

	if behavior.shot_timer_id in finished_timers {
		world.add_entity(
			ctx.world,
			bullet_prefab(ctx.self.transform.position, -500, "Enemy Bullet"),
		)
	}
}

@(private = "file")
on_update: world.Lifecycle_Proc : proc(ctx: world.Behavior_Context) {
	transform := &ctx.self.transform
	behavior := world.get_behavior(ctx.self^, Enemy_Behavior).?

	move(transform, behavior^, ctx.delta_time)
}

@(private = "file")
move :: proc(
	transform: ^components.Transform_Component,
	behavior: Enemy_Behavior,
	dt: f64,
) {
	transform.position.y += behavior.speed * dt
}

@(private = "file")
on_collision: world.On_Collision_Proc : proc(
	ctx: world.Behavior_Context,
	other: ^world.Entity,
) {
	if other.tag == "Player Bullet" || other.tag == "Laser" {
		world.remove_entity(ctx.world, ctx.self.token)
	}
}

