package example_shmup_prefabs

import "core:log"
import v2 "kmath:vector2"
import "karakuri:world"
import "karakuri:components"

laser_prefab :: proc(start_position: v2.Vector2) -> world.Entity_Payload {
	behavior := new(Laser_Behavior)
	behavior^ = Laser_Behavior {
		on_start   = on_start,
		on_update  = on_update,
		on_destroy = on_destroy,
	}

	return world.Entity_Payload {
		tag = "Laser",
		transform = components.Transform_Component {
			position = {start_position.x, start_position.y / 2},
			scale = {1, 10},
		},
		box_collider = components.Box_Collider_Component{},
		sprite = components.Sprite_Component{sprite_name = "laser_blue"},
		behavior = behavior,
	}
}

@(private = "file")
Laser_Behavior :: struct {
	using behavior: world.Behavior,
	player_token:   world.Token,
}

@(private = "file")
on_start :: proc(ctx: world.Behavior_Context) {
	log.info("Laser dtarted")

	behavior := world.get_behavior(ctx.self^, Laser_Behavior).?

	behavior.player_token = world.find_with_tag(ctx.world^, "Player").?.token
}

@(private = "file")
on_destroy :: proc(ctx: world.Behavior_Context) {
	log.info("Laser destroyed")
}

@(private = "file")
on_update :: proc(ctx: world.Behavior_Context) {
	behavior := world.get_behavior(ctx.self^, Laser_Behavior).?

	if !world.is_alive(ctx.world^, behavior.player_token) {
		world.remove_entity(ctx.world, ctx.self.token)
		return
	}

	player_position :=
		world.get_entity(ctx.world, behavior.player_token).?.transform.position

	ctx.self.transform.position = {player_position.x, player_position.y - 250}
}

