package example_shmup_prefabs

import "core:log"
import v2 "kmath:vector2"
import "karakuri:components"
import "karakuri:input_manager"
import "karakuri:world"

player_prefab :: proc() -> world.Entity_Payload {
	player_behavior := new(Player_Behavior)
	player_behavior^ = Player_Behavior {
		speed        = 500,
		on_start     = on_start,
		on_update    = on_update,
		on_destroy   = on_destroy,
		on_collision = on_collision,
	}

	return world.Entity_Payload {
		tag = "Player",
		transform = components.Transform_Component {
			position = v2.Vector2{0, 0},
			rotation = 0,
		},
		box_collider = components.Box_Collider_Component{size = {30, 50}},
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
	shoot(ctx.world, transform^)
}

@(private = "file")
on_start: world.Lifecycle_Proc : proc(ctx: world.Behavior_Context) {
	log.info("Player started")
}

@(private = "file")
on_destroy: world.Lifecycle_Proc : proc(ctx: world.Behavior_Context) {
	log.info("Player destroyed")
}

@(private = "file")
move :: proc(
	transform: ^components.Transform_Component,
	behavior: ^Player_Behavior,
	dt: f64,
) {
	disposition := behavior.speed * dt

	if input_manager.is_key_down(.W) {
		transform.position.y -= disposition
	}
	if input_manager.is_key_down(.A) {
		transform.position.x -= disposition
	}
	if input_manager.is_key_down(.S) {
		transform.position.y += disposition
	}
	if input_manager.is_key_down(.D) {
		transform.position.x += disposition
	}
}

@(private = "file")
shoot :: proc(w: ^world.World, transform: components.Transform_Component) {
	if input_manager.is_key_pressed(.SPACE) ||
	   input_manager.is_key_pressed(.ENTER) {
		world.add_entity(
			w,
			bullet_prefab(transform.position, 700, "Player Bullet"),
		)
	}

	if input_manager.is_key_pressed(.K) {
		world.add_entity(w, laser_prefab(transform.position))
	}

	if input_manager.is_key_released(.K) {
		laser, laser_ok := world.find_with_tag(w^, "Laser").?
		if !laser_ok {
			return
		}

		world.remove_entity(w, laser.token)
	}
}

@(private = "file")
on_collision: world.On_Collision_Proc : proc(
	ctx: world.Behavior_Context,
	other: ^world.Entity,
) {
	if other.tag == "Enemy Bullet" {
		world.remove_entity(ctx.world, ctx.self.token)
	}
}

