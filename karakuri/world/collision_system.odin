package karakuri_world

import rl "vendor:raylib"
import "ktimer:timer"

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

		for j in i + 1 ..< len(entities) {
			other := &entities[j]

			other_transform := &other.transform
			other_box_collider, other_box_collider_ok := &other.box_collider.?

			if !other_box_collider_ok {
				continue
			}

			are_colliding := rl.CheckCollisionRecs(
				rl.Rectangle {
					x = f32(
						transform.position.x +
						box_collider.offset.x -
						box_collider.size.x / 2 * transform.scale.x,
					),
					y = f32(
						transform.position.y +
						box_collider.offset.y -
						box_collider.size.y / 2 * transform.scale.y,
					),
					width = f32(box_collider.size.x * transform.scale.x),
					height = f32(box_collider.size.y * transform.scale.y),
				},
				rl.Rectangle {
					x = f32(
						other_transform.position.x +
						other_box_collider.offset.x -
						other_box_collider.size.x /
							2 *
							other_transform.scale.x,
					),
					y = f32(
						other_transform.position.y +
						other_box_collider.offset.y -
						other_box_collider.size.y /
							2 *
							other_transform.scale.y,
					),
					width = f32(
						other_box_collider.size.x * other_transform.scale.x,
					),
					height = f32(
						other_box_collider.size.y * other_transform.scale.y,
					),
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

