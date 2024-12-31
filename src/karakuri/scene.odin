#+private

package karakuri

import "../kec"
import fps "../kwindow/fps_manager"
import input "../kwindow/input_manager"
import renderer "../kwindow/renderer"

Scene :: struct {
	spawner:  Spawner,
	registry: kec.Registry,
}

new_scene :: proc(initial_entities: [dynamic]Component_Bundle) -> Scene {
	scene := Scene {
		spawner  = new_spawner(),
		registry = kec.new_registry(),
	}

	sync_add_entities(
		initial_entities[:],
		&scene.registry,
		&scene.spawner,
		fps.get_delta_time(),
	)

	return scene
}

destroy_scene :: proc(s: Scene) {
	destroy_spawner(s.spawner)

	kec.destroy_registry(s.registry)
}

update_scene :: proc(s: ^Scene, renderer_info: ^renderer.Renderer_Info) {
	delta_time := fps.get_delta_time()

	sync_with_registry(s, delta_time)
	update_entities(s, delta_time)
	render_entities(s, renderer_info)
}

@(private = "file")
sync_with_registry :: proc(scene: ^Scene, delta_time: f64) {
	sync_remove_entities(
		scene.spawner.entities_to_remove[:],
		&scene.registry,
		&scene.spawner,
		delta_time,
	)
	clear(&scene.spawner.entities_to_remove)

	sync_add_entities(
		scene.spawner.entities_to_add[:],
		&scene.registry,
		&scene.spawner,
		delta_time,
	)
	clear(&scene.spawner.entities_to_add)
}

@(private = "file")
sync_add_entities :: proc(
	entities_to_add: []Component_Bundle,
	registry: ^kec.Registry,
	spawner: ^Spawner,
	delta_time: f64,
) {
	behavior_ctx := make_behavior_context(delta_time, spawner)

	for bundle in entities_to_add {
		entity := kec.create_entity(registry)

		if transform, ok := bundle.transform.?; ok {
			kec.add_component(registry, entity, transform)
		} else {
			kec.add_component(registry, entity, new_transform_component())
		}

		if shape, ok := bundle.shape.?; ok {
			kec.add_component(registry, entity, shape)
		}

		if behavior, ok := bundle.behavior.?; ok {
			kec.add_component(registry, entity, behavior)

			if on_start, ok := behavior.on_start.?; ok {
				behavior_ctx.entity = entity

				on_start(behavior_ctx)
			}
		}
	}
}

@(private = "file")
sync_remove_entities :: proc(
	entities_to_remove: []kec.Entity,
	registry: ^kec.Registry,
	spawner: ^Spawner,
	delta_time: f64,
) {
	behavior_ctx := make_behavior_context(delta_time, spawner)

	for entity in entities_to_remove {
		behavior := kec.get_component(registry^, entity, Behavior_Component)
		if behavior != nil {
			if on_destroy, ok := behavior.on_destroy.?; ok {
				on_destroy(behavior_ctx)
			}
		}

		kec.destroy_entity(registry, entity)
	}
}

@(private = "file")
update_entities :: proc(scene: ^Scene, delta_time: f64) {
	updatable_query := kec.query_start()
	kec.query_with(Behavior_Component, &updatable_query, scene.registry)
	updatable_entities := kec.query_submit(updatable_query, scene.registry)
	defer delete(updatable_entities)

	ctx := make_behavior_context(delta_time, &scene.spawner)

	for entity in updatable_entities {
		behavior := kec.get_component(
			scene.registry,
			entity,
			Behavior_Component,
		)

		// TODO: It should not be possible to get nils.
		// It's possible now, it's even expected, but the design is bad.
		if behavior == nil {
			continue
		}

		if on_update, ok := behavior.on_update.?; ok {
			ctx.entity = entity

			on_update(ctx)
		}
	}
}

@(private = "file")
render_entities :: proc(s: ^Scene, renderer_info: ^renderer.Renderer_Info) {
	renderable := kec.query_start()
	kec.query_with(Transform_Component, &renderable, s.registry)
	kec.query_with(Shape_Component, &renderable, s.registry)
	renderable_entities := kec.query_submit(renderable, s.registry)
	defer delete(renderable_entities)

	renderer.start_drawing(renderer_info)

	for entity in renderable_entities {
		transform := kec.get_component(s.registry, entity, Transform_Component)
		shape := kec.get_component(s.registry, entity, Shape_Component)

		// TODO: It should not be possible to get nils.
		// It's possible now, it's even expected, but the design is bad.
		if transform == nil || shape == nil {
			continue
		}

		renderer.draw_rectangle(
			renderer_info = renderer_info^,
			position = transform.position,
			width = shape.width,
			height = shape.height,
			scale = transform.scale,
			rotation = transform.rotation,
			color = shape.color,
		)
	}

	renderer.finish_drawing()
}

@(private = "file")
make_behavior_context :: proc(dt: f64, spawner: ^Spawner) -> Behavior_Context {
	return Behavior_Context {
		dt = dt,
		spawner = spawner,
		input = {
			is_key_down = input.is_key_down,
			is_key_up = input.is_key_up,
			is_key_pressed = input.is_key_pressed,
			is_key_released = input.is_key_released,
		},
	}
}
