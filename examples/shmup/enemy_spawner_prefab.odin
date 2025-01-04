package shmup

import "core:math/rand"
import "karakuri:karakuri/components"
import "karakuri:kmath"

enemy_spawner_prefab :: proc() -> components.Component_Bundle {
	return components.Component_Bundle {
		behavior = components.Behavior_Component {
			on_start = on_start,
			on_destroy = on_destroy,
			on_timer = on_timer,
		},
	}
}

spawn_interval_id: uint

@(private = "file")
on_start: components.Lifecycle_Proc : proc(ctx: components.Behavior_Context) {
	spawn_interval_id = ctx.timer.set_interval(ctx.timer.timer_info, 2000)
}

@(private = "file")
on_destroy: components.Lifecycle_Proc : proc(
	ctx: components.Behavior_Context,
) {
	ctx.timer.clear_interval(ctx.timer.timer_info, spawn_interval_id)
}

on_timer: components.On_Timer_Proc : proc(
	ctx: components.Behavior_Context,
	finished_timers: map[uint]struct {},
) {
	if spawn_interval_id in finished_timers {
		position := kmath.Vector2 {
			f64(rand.float64_range(-200, 200)),
			-320 + 30,
		}
		ctx.spawner.add_entity(ctx.spawner, enemy_prefab(position))
	}
}

