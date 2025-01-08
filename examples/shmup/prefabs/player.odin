package example_shmup_prefabs

import "core:fmt"
import "karakuri:components"
import "karakuri:input_manager"
import "karakuri:world"
import v2 "kmath:vector2"
import "kutils:color"
import "kutils:keyboard"

player_prefab :: proc() -> world.Entity_Payload {
	player_behavior := new(Player_Behavior)
	player_behavior^ = Player_Behavior {
		speed      = 500,
		on_start   = on_start,
		on_update  = on_update,
		on_destroy = on_destroy,
	}

	return world.Entity_Payload {
		tag = "Sonic",
		transform = components.Transform_Component {
			position = v2.Vector2{0, 0},
			scale = v2.Vector2{1, 1},
			rotation = 0,
		},
		shape = components.Shape_Component {
			size = v2.Vector2{30, 50},
			color = color.Yellow,
		},
		behavior = player_behavior,
	}
}

@(private = "file")
Player_Behavior :: struct {
	using behavior: world.Behavior,
	speed:          f64,
}

@(private = "file")
on_update: world.Lifecycle_Proc : proc(ctx: world.Behavior_Context) {
	transform := &ctx.self.transform
	behavior := world.get_behavior(ctx.self^, Player_Behavior).?

	move(transform, behavior, ctx.delta_time)
}

@(private = "file")
on_start: world.Lifecycle_Proc : proc(ctx: world.Behavior_Context) {
	fmt.println("Sonic started")
}

@(private = "file")
on_destroy: world.Lifecycle_Proc : proc(ctx: world.Behavior_Context) {
	fmt.println("Sonic destroyed")
}

@(private = "file")
move :: proc(
	transform: ^components.Transform_Component,
	behavior: ^Player_Behavior,
	dt: f64,
) {
	disposition := behavior.speed * dt

	if input_manager.is_key_down(keyboard.Key.W) {
		transform.position.y -= disposition
	}
	if input_manager.is_key_down(keyboard.Key.A) {
		transform.position.x -= disposition
	}
	if input_manager.is_key_down(keyboard.Key.S) {
		transform.position.y += disposition
	}
	if input_manager.is_key_down(keyboard.Key.D) {
		transform.position.x += disposition
	}
}
