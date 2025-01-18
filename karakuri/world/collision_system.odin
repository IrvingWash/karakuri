package karakuri_world

import rl "vendor:raylib"
import "ktimer:timer"
import v2 "kmath:vector2"

collision_system :: proc(
	entities: []Entity,
	world: ^World,
	delta_time: f64,
	timer_info: ^timer.Timer_Info,
) {
	for i in 0 ..< len(entities) {
		entity := &entities[i]

		transform := &entity.transform
		box_collider, box_collider_ok := &entity.box_collider.?
		if !box_collider_ok {
			continue
		}

		behavior, behavior_ok := entity.behavior.?

		origin: v2.Vector2 = ---
		if sprite, ok := &entity.sprite.?; ok {
			origin = sprite.origin.? or_else box_collider.size / 2
		} else {
			origin = box_collider.size / 2
		}

		position :=
			transform.position + box_collider.offset - origin * transform.scale
		size := box_collider.size * transform.scale

		for j in i + 1 ..< len(entities) {
			other := &entities[j]

			other_transform := &other.transform
			other_box_collider, other_box_collider_ok := &other.box_collider.?

			if !other_box_collider_ok {
				continue
			}

			other_origin: v2.Vector2 = ---
			if other_sprite, ok := &other.sprite.?; ok {
				other_origin =
					other_sprite.origin.? or_else other_box_collider.size / 2
			} else {
				other_origin = box_collider.size / 2
			}

			other_position :=
				other_transform.position +
				other_box_collider.offset -
				other_origin * other_transform.scale
			other_size := other_box_collider.size * other_transform.scale

			are_colliding := rl.CheckCollisionRecs(
				rl.Rectangle {
					x = f32(position.x),
					y = f32(position.y),
					width = f32(size.x),
					height = f32(size.y),
				},
				rl.Rectangle {
					x = f32(other_position.x),
					y = f32(other_position.y),
					width = f32(other_size.x),
					height = f32(other_size.y),
				},
			)

			if are_colliding {
				if behavior_ok {
					if on_collision, on_collision_ok := behavior.on_collision.?;
					   on_collision_ok {
						on_collision(
							make_behavior_context(
								entity,
								delta_time,
								world,
								timer_info,
							),
							other,
						)
					}
				}

				other_behavior, other_behavior_ok := other.behavior.?
				if other_behavior_ok {
					if on_collision, on_collision_ok := other_behavior.on_collision.?;
					   on_collision_ok {
						on_collision(
							make_behavior_context(
								other,
								delta_time,
								world,
								timer_info,
							),
							entity,
						)
					}
				}
			}
		}
	}
}

