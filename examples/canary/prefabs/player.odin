package example_canary_prefabs

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
		speed = 300,
		on_start = proc(ctx: world.Behavior_Context) {
			fmt.println("Sonic started")
		},
		on_update = proc(ctx: world.Behavior_Context) {
			transform := &ctx.self.transform
			behavior := world.get_behavior(ctx.self^, Player_Behavior).?

			disposition := behavior.speed * ctx.delta_time

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
		},
		on_destroy = proc(ctx: world.Behavior_Context) {
			fmt.println("Sonic destroyed")
		},
	}

	return world.Entity_Payload {
		tag = "Sonic",
		transform = components.Transform_Component {
			position = v2.Vector2{0, 0},
			scale = v2.Vector2{1, 1},
			rotation = 0,
		},
		shape = components.Shape_Component {
			size = v2.Vector2{100, 100},
			color = color.White,
		},
		behavior = player_behavior,
	}
}

Player_Behavior :: struct {
	using behavior: world.Behavior,
	speed:          f64,
}

