package scene

import "../../kec"
import "../../kwindow/fps_manager"
import "../../kwindow/input_manager"
import "../../kwindow/renderer"
import "../components"
import "../systems"

Scene_Info :: struct {
	spawner_info: components.Spawner_Info,
	registry:     kec.Registry,
}

new_scene :: proc(
	initial_entities: [dynamic]components.Component_Bundle,
) -> Scene_Info {
	defer delete(initial_entities)

	scene_info := Scene_Info {
		spawner_info = components.new_spawner_info(),
		registry     = kec.new_registry(),
	}

	sync_add_entities(
		initial_entities[:],
		&scene_info.registry,
		&scene_info.spawner_info,
		fps_manager.get_delta_time(),
	)

	return scene_info
}

destroy_scene_info :: proc(scene_info: Scene_Info) {
	components.destroy_spawner_info(scene_info.spawner_info)
	kec.destroy_registry(scene_info.registry)
}

update :: proc(
	scene_info: ^Scene_Info,
	renderer_info: ^renderer.Renderer_Info,
) {
	delta_time := fps_manager.get_delta_time()

	sync_with_registry(scene_info, delta_time)

	update_entities(scene_info, delta_time)

	render_entities(scene_info, renderer_info)
}

@(private = "file")
sync_with_registry :: proc(scene_info: ^Scene_Info, delta_time: f64) {
	sync_remove_entities(
		scene_info.spawner_info.entities_to_remove[:],
		&scene_info.registry,
		&scene_info.spawner_info,
		delta_time,
	)
	clear(&scene_info.spawner_info.entities_to_remove)

	sync_add_entities(
		scene_info.spawner_info.entities_to_add[:],
		&scene_info.registry,
		&scene_info.spawner_info,
		delta_time,
	)
	clear(&scene_info.spawner_info.entities_to_add)
}

@(private = "file")
sync_add_entities :: proc(
	entities_to_add: []components.Component_Bundle,
	registry: ^kec.Registry,
	spawner_info: ^components.Spawner_Info,
	delta_time: f64,
) {
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
				behavior_ctx := make_behavior_context(
					delta_time,
					spawner_info,
					registry^,
					entity,
				)

				on_start(behavior_ctx)
			}
		}

		if rigid_body, ok := bundle.rigid_body.?; ok {
			kec.add_component(registry, entity, rigid_body)
		}

		if tag, ok := bundle.tag.?; ok {
			kec.add_component(registry, entity, tag)
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
	for entity in entities_to_remove {
		behavior := kec.get_component(
			registry^,
			entity,
			components.Behavior_Component,
		)

		if behavior != nil {
			if on_destroy, ok := behavior.on_destroy.?; ok {
				behavior_ctx := make_behavior_context(
					delta_time,
					spawner_info,
					registry^,
					entity,
				)

				on_destroy(behavior_ctx)
			}
		}

		kec.destroy_entity(registry, entity)
	}
}

@(private = "file")
update_entities :: proc(scene_info: ^Scene_Info, delta_time: f64) {
	updatable_query := kec.query_start()
	kec.query_with(
		components.Behavior_Component,
		&updatable_query,
		scene_info.registry,
	)
	updatable_entities := kec.query_submit(
		updatable_query,
		scene_info.registry,
	)
	defer delete(updatable_entities)

	for entity in updatable_entities {
		behavior := kec.get_component(
			scene_info.registry,
			entity,
			components.Behavior_Component,
		)

		if on_update, ok := behavior.on_update.?; ok {
			ctx := make_behavior_context(
				delta_time,
				&scene_info.spawner_info,
				scene_info.registry,
				entity,
			)

			on_update(ctx)
		}
	}

	systems.physics_system(
		scene_info.registry,
		delta_time,
		make_behavior_context,
		&scene_info.spawner_info,
	)
}

@(private = "file")
render_entities :: proc(
	scene_info: ^Scene_Info,
	renderer_info: ^renderer.Renderer_Info,
) {
	renderable := kec.query_start()
	kec.query_with(
		components.Transform_Component,
		&renderable,
		scene_info.registry,
	)
	kec.query_with(
		components.Shape_Component,
		&renderable,
		scene_info.registry,
	)
	renderable_entities := kec.query_submit(renderable, scene_info.registry)
	defer delete(renderable_entities)

	renderer.start_drawing(renderer_info)
	defer renderer.finish_drawing()

	for entity in renderable_entities {
		transform := kec.get_component(
			scene_info.registry,
			entity,
			components.Transform_Component,
		)
		shape := kec.get_component(
			scene_info.registry,
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
}

@(private = "file")
make_behavior_context :: proc(
	dt: f64,
	spawner_info: ^components.Spawner_Info,
	registry: kec.Registry,
	entity: kec.Entity,
) -> components.Behavior_Context {
	return components.Behavior_Context {
		entity = entity,
		dt = dt,
		spawner = spawner_info,
		registry = registry,
		input = {
			is_key_down = input_manager.is_key_down,
			is_key_up = input_manager.is_key_up,
			is_key_pressed = input_manager.is_key_pressed,
			is_key_released = input_manager.is_key_released,
		},
	}
}

