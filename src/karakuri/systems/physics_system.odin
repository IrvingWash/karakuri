package systems

import "../../kec"
import "../components"
import rl "vendor:raylib"

physics_system :: proc(
	registry: kec.Registry,
	dt: f64,
	ctx_maker: proc(
		dt: f64,
		spawner_info: ^components.Spawner_Info,
		registry: kec.Registry,
		entity: kec.Entity,
	) -> components.Behavior_Context,
	spawner_info: ^components.Spawner_Info,
) {
	move_entities(registry, dt)
	collide_entities(registry, ctx_maker, dt, spawner_info)
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
	ctx_maker: proc(
		dt: f64,
		spawner_info: ^components.Spawner_Info,
		registry: kec.Registry,
		entity: kec.Entity,
	) -> components.Behavior_Context,
	dt: f64,
	spawner_info: ^components.Spawner_Info,
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

				if behavior != nil {
					if on_collision, ok := behavior.on_collision.?; ok {
						ctx := ctx_maker(dt, spawner_info, registry, entity)

						on_collision(ctx, other)
					}
				}
				if other_behavior != nil {
					if on_collision, ok := other_behavior.on_collision.?; ok {
						ctx := ctx_maker(dt, spawner_info, registry, other)

						on_collision(ctx, entity)
					}
				}
			}
		}
	}
}

