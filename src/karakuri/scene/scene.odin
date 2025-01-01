package karakuri

import "../../kec"
import "../../kwindow/fps_manager"
import "../../kwindow/input_manager"
import "../../kwindow/renderer"
import "../components"

Scene :: struct {
	spawner_info: components.Spawner_Info,
	registry:     kec.Registry,
}

new_scene :: proc(
	initial_entities: [dynamic]components.Component_Bundle,
) -> Scene {
	defer delete(initial_entities)

	scene := Scene {
		spawner_info = components.new_spawner_info(),
		registry     = kec.new_registry(),
	}

	sync_add_entities(
		initial_entities[:],
		&scene.registry,
		&scene.spawner_info,
		fps_manager.get_delta_time(),
	)

	return scene
}

destroy_scene :: proc(s: Scene) {
	components.destroy_spawner_info(s.spawner_info)
	kec.destroy_registry(s.registry)
}

update_scene :: proc(scene: ^Scene, renderer_info: ^renderer.Renderer_Info) {
	delta_time := fps_manager.get_delta_time()

	sync_with_registry(scene, delta_time)

	update_entities(scene, delta_time)

	render_entities(scene, renderer_info)
}

@(private = "file")
sync_with_registry :: proc(scene: ^Scene, delta_time: f64) {
	sync_remove_entities(
		scene.spawner_info.entities_to_remove[:],
		&scene.registry,
		&scene.spawner_info,
		delta_time,
	)
	clear(&scene.spawner_info.entities_to_remove)

	sync_add_entities(
		scene.spawner_info.entities_to_add[:],
		&scene.registry,
		&scene.spawner_info,
		delta_time,
	)
	clear(&scene.spawner_info.entities_to_add)
}

@(private = "file")
sync_add_entities :: proc(
	entities_to_add: []components.Component_Bundle,
	registry: ^kec.Registry,
	spawner_info: ^components.Spawner_Info,
	delta_time: f64,
) {
	behavior_ctx := make_behavior_context(delta_time, spawner_info)

	for bundle in entities_to_add {
		entity := kec.create_entity(registry)

		if transform, ok := bundle.transform.?; ok {
			kec.add_component(registry, entity, transform)
		} else {
			kec.add_component(
				registry,
				entity,
				components.new_transform_component(),
			)
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
	spawner_info: ^components.Spawner_Info,
	delta_time: f64,
) {
	behavior_ctx := make_behavior_context(delta_time, spawner_info)

	for entity in entities_to_remove {
		behavior := kec.get_component(
			registry^,
			entity,
			components.Behavior_Component,
		)

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
	kec.query_with(
		components.Behavior_Component,
		&updatable_query,
		scene.registry,
	)
	updatable_entities := kec.query_submit(updatable_query, scene.registry)
	defer delete(updatable_entities)

	ctx := make_behavior_context(delta_time, &scene.spawner_info)

	for entity in updatable_entities {
		behavior := kec.get_component(
			scene.registry,
			entity,
			components.Behavior_Component,
		)

		if on_update, ok := behavior.on_update.?; ok {
			ctx.entity = entity

			on_update(ctx)
		}
	}
}

@(private = "file")
render_entities :: proc(s: ^Scene, renderer_info: ^renderer.Renderer_Info) {
	renderable := kec.query_start()
	kec.query_with(components.Transform_Component, &renderable, s.registry)
	kec.query_with(components.Shape_Component, &renderable, s.registry)
	renderable_entities := kec.query_submit(renderable, s.registry)
	defer delete(renderable_entities)

	renderer.start_drawing(renderer_info)

	for entity in renderable_entities {
		transform := kec.get_component(
			s.registry,
			entity,
			components.Transform_Component,
		)
		shape := kec.get_component(
			s.registry,
			entity,
			components.Shape_Component,
		)

		renderer.draw_rectangle(
			renderer_info = renderer_info^,
			position = transform.position,
			width = shape.size.x,
			height = shape.size.y,
			scale = transform.scale,
			rotation = transform.rotation,
			color = shape.color,
		)
	}

	renderer.finish_drawing()
}

@(private = "file")
make_behavior_context :: proc(
	dt: f64,
	spawner_info: ^components.Spawner_Info,
) -> components.Behavior_Context {
	return components.Behavior_Context {
		dt = dt,
		spawner = spawner_info,
		input = {
			is_key_down = input_manager.is_key_down,
			is_key_up = input_manager.is_key_up,
			is_key_pressed = input_manager.is_key_pressed,
			is_key_released = input_manager.is_key_released,
		},
	}
}
