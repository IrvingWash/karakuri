package systems

import "../../kec"
import "../components"
import rl "vendor:raylib"

physics_system :: proc(
	registry: kec.Registry,
	behavior_context: ^components.Behavior_Context,
) {
	move_entities(registry, behavior_context.dt)
	collide_entities(registry, behavior_context)
}

@(private = "file")
move_entities :: proc(registry: kec.Registry, dt: f64) {
	movable_query := kec.query_start()
	kec.query_with(components.Rigid_Body_Component, &movable_query, registry)
	movable_entities := kec.query_submit(movable_query, registry)
	defer delete(movable_entities)

	for entity in movable_entities {
		transform := kec.get_component(
			registry,
			entity,
			components.Transform_Component,
		)
		rigid_body := kec.get_component(
			registry,
			entity,
			components.Rigid_Body_Component,
		)

		transform.position += rigid_body.velocity * dt
	}
}

@(private = "file")
collide_entities :: proc(
	registry: kec.Registry,
	behavior_context: ^components.Behavior_Context,
) {
	collidable_query := kec.query_start()
	kec.query_with(components.Shape_Component, &collidable_query, registry)
	collideable_entities := kec.query_submit(collidable_query, registry)
	defer delete(collideable_entities)

	for i in 0 ..< len(collideable_entities) {
		entity := collideable_entities[i]

		transform := kec.get_component(
			registry,
			entity,
			components.Transform_Component,
		)
		shape := kec.get_component(
			registry,
			entity,
			components.Shape_Component,
		)

		for j in i + 1 ..< len(collideable_entities) {
			other := collideable_entities[j]

			other_transform := kec.get_component(
				registry,
				other,
				components.Transform_Component,
			)
			other_shape := kec.get_component(
				registry,
				other,
				components.Shape_Component,
			)

			are_colliding := rl.CheckCollisionRecs(
				rl.Rectangle {
					x = f32(transform.position.x),
					y = f32(transform.position.y),
					width = f32(shape.size.x),
					height = f32(shape.size.y),
				},
				rl.Rectangle {
					x = f32(other_transform.position.x),
					y = f32(other_transform.position.y),
					width = f32(other_shape.size.x),
					height = f32(other_shape.size.y),
				},
			)

			if are_colliding {
				behavior := kec.get_component(
					registry,
					entity,
					components.Behavior_Component,
				)
				other_behavior := kec.get_component(
					registry,
					other,
					components.Behavior_Component,
				)

				if on_collision, ok := behavior.on_collision.?; ok {
					behavior_context.entity = entity
					on_collision(behavior_context^, other)
				}
				if on_collision, ok := other_behavior.on_collision.?; ok {
					behavior_context.entity = other
					on_collision(behavior_context^, entity)
				}
			}
		}
	}
}
