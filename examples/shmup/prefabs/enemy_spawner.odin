package example_shmup_prefabs

import "core:fmt"
import "karakuri:world"
import "ktimer:timer"

enemy_spawner_prefab :: proc() -> world.Entity_Payload {
	enemy_spawner := new(Enemy_Spawner)
	enemy_spawner^ = Enemy_Spawner {
		spawn_interval_id = 0,
		on_start          = on_start,
		on_destroy        = on_destroy,
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

