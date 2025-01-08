package example_shmup_prefabs

import "core:fmt"
import "core:math/rand"
import v2 "kmath:vector2"
import "ktimer:timer"
import "karakuri:world"

enemy_spawner_prefab :: proc() -> world.Entity_Payload {
	enemy_spawner := new(Enemy_Spawner)
	enemy_spawner^ = Enemy_Spawner {
		spawn_interval_id = 0,
		on_start          = on_start,
		on_destroy        = on_destroy,
		on_timer          = on_timer,
	}

	return world.Entity_Payload {
		tag = "Enemy Spawner",
		behavior = enemy_spawner,
	}
}

@(private = "file")
Enemy_Spawner :: struct {
	using behavior:    world.Behavior,
	spawn_interval_id: uint,
}

@(private = "file")
on_start: world.Lifecycle_Proc : proc(ctx: world.Behavior_Context) {
	fmt.println("Enemy Spawner started")

	behavior := world.get_behavior(ctx.self^, Enemy_Spawner).?

	behavior.spawn_interval_id = timer.set_interval(ctx.timer_info, 2000)
}

@(private = "file")
on_destroy: world.Lifecycle_Proc : proc(ctx: world.Behavior_Context) {
	fmt.println("Enemy Spawner destroyed")

	behavior := world.get_behavior(ctx.self^, Enemy_Spawner).?

	timer.clear_interval(ctx.timer_info, behavior.spawn_interval_id)
}

@(private = "file")
on_timer: world.On_Timer_Proc : proc(
	ctx: world.Behavior_Context,
	finished_timers: map[uint]struct {},
) {
	behavior := world.get_behavior(ctx.self^, Enemy_Spawner).?

	if behavior.spawn_interval_id in finished_timers {
		enemy_position := v2.Vector2{rand.float64_range(-200, 200), -320 + 30}
		world.add_entity(ctx.world, enemy_prefab(enemy_position))
	}
}

