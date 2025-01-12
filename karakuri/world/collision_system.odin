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
		shape, shape_ok := &entity.shape.?
		if !shape_ok {
			continue
		}

		behavior, behavior_ok := entity.behavior.?

		for j in i + 1 ..< len(entities) {
			other := &entities[j]

			other_transform := &other.transform
			other_shape, other_shape_ok := &other.shape.?

			if !other_shape_ok {
				continue
			}

			are_colliding := rl.CheckCollisionRecs(
				rl.Rectangle {
					x = f32(transform.position.x - shape.size.x / 2),
					y = f32(transform.position.y - shape.size.y / 2),
					width = f32(shape.size.x),
					height = f32(shape.size.y),
				},
				rl.Rectangle {
					x = f32(
						other_transform.position.x - other_shape.size.x / 2,
					),
					y = f32(
						other_transform.position.y - other_shape.size.y / 2,
					),
					width = f32(other_shape.size.x),
					height = f32(other_shape.size.y),
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
