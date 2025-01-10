package example_shmup_prefabs

import "core:fmt"
import v2 "kmath:vector2"
import "kutils:color"
import "karakuri:world"
import "karakuri:components"

laser_prefab :: proc(start_position: v2.Vector2) -> world.Entity_Payload {
	behavior := new(Laser_Behavior)
	behavior ^= Laser_Behavior{
		on_start = on_start,
		on_update = on_update,
		on_destroy = on_destroy,
	}

	return world.Entity_Payload {
		tag = "Laser",
		transform = components.Transform_Component{
			position = {start_position.x, start_position.y / 2},
			scale = v2.Unit,
		},
		shape = components.Shape_Component {
			size = v2.Vector2{20, 1000},
			color = color.Red,
		},
		behavior = behavior,
	}
}

@(private = "file")
Laser_Behavior :: struct {
	using behavior: world.Behavior,
	player_token: world.Token,
}

@(private = "file")
on_start :: proc(ctx: world.Behavior_Context) {
	fmt.println("Laser dtarted")

	behavior := world.get_behavior(ctx.self^, Laser_Behavior).?

	behavior.player_token = world.find_with_tag(ctx.world^, "Player").?.token
}

@(private = "file")
on_destroy :: proc(ctx: world.Behavior_Context) {
	fmt.println("Laser destroyed")
}

@(private = "file")
on_update :: proc(ctx: world.Behavior_Context) {
	behavior := world.get_behavior(ctx.self^, Laser_Behavior).?

	if !world.is_alive(ctx.world^, behavior.player_token) {
		return
	}

	player_position := world.get_entity(ctx.world, behavior.player_token).?.transform.position

	ctx.self.transform.position = {player_position.x, player_position.y - 500}
}
