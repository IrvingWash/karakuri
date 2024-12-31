package karakuri

import "../kec"
import fps "../kwindow/fps_manager"
import input "../kwindow/input_manager"
import renderer "../kwindow/renderer"
import comp "./components"

Scene :: struct {
	registry:            kec.Registry,
	entities_to_add:     [dynamic]comp.Component_Bundle,
	entities_to_remove:  [dynamic]kec.Entity,
	entities_to_start:   [dynamic]kec.Entity,
	entities_to_destroy: [dynamic]kec.Entity,
}

scene_new :: proc(initial_entities: [dynamic]comp.Component_Bundle) -> Scene {
	scene := Scene {
		registry            = kec.new_registry(),
		entities_to_add     = make([dynamic]comp.Component_Bundle),
		entities_to_remove  = make([dynamic]kec.Entity),
		entities_to_start   = make([dynamic]kec.Entity),
		entities_to_destroy = make([dynamic]kec.Entity),
	}

	sync_add_entities(
		initial_entities[:],
		&scene.registry,
		fps.get_delta_time(),
	)
	delete(initial_entities)

	return scene
}

scene_destroy :: proc(s: Scene) {
	kec.destroy_registry(s.registry)

	delete(s.entities_to_add)
	delete(s.entities_to_remove)
	delete(s.entities_to_start)
	delete(s.entities_to_destroy)
}

scene_add_entity :: proc(bundle: comp.Component_Bundle, scene: ^Scene) {
	append(&scene.entities_to_add, bundle)
}

scene_remove_entity :: proc(entity: kec.Entity, scene: ^Scene) {
	append(&scene.entities_to_remove, entity)
}

@(private)
scene_update :: proc(s: ^Scene, renderer_info: ^renderer.Renderer_Info) {
	delta_time := fps.get_delta_time()

	sync_with_registry(s, delta_time)
	update_entities(s, delta_time)
	render_entities(s, renderer_info)
}

@(private = "file")
sync_with_registry :: proc(scene: ^Scene, delta_time: f64) {
	sync_remove_entities(
		scene.entities_to_remove[:],
		&scene.registry,
		delta_time,
	)
	clear(&scene.entities_to_remove)

	sync_add_entities(scene.entities_to_add[:], &scene.registry, delta_time)
	clear(&scene.entities_to_add)
}

@(private = "file")
sync_add_entities :: proc(
	entities_to_add: []comp.Component_Bundle,
	registry: ^kec.Registry,
	delta_time: f64,
) {
	behavior_ctx := make_behavior_context(delta_time)

	for bundle in entities_to_add {
		entity := kec.create_entity(registry)

		if transform, ok := bundle.transform.?; ok {
			kec.add_component(registry, entity, transform)
		} else {
			kec.add_component(registry, entity, comp.new_transform_component())
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
	delta_time: f64,
) {
	behavior_ctx := make_behavior_context(delta_time)

	for entity in entities_to_remove {
		behavior := kec.get_component(
			registry^,
			entity,
			comp.Behavior_Component,
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
	kec.query_with(comp.Behavior_Component, &updatable_query, scene.registry)
	updatable_entities := kec.query_submit(updatable_query, scene.registry)
	defer delete(updatable_entities)

	ctx := make_behavior_context(delta_time)

	for entity in updatable_entities {
		behavior := kec.get_component(
			scene.registry,
			entity,
			comp.Behavior_Component,
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
	kec.query_with(comp.Transform_Component, &renderable, s.registry)
	kec.query_with(comp.Shape_Component, &renderable, s.registry)
	renderable_entities := kec.query_submit(renderable, s.registry)
	defer delete(renderable_entities)

	renderer.start_drawing(renderer_info)

	for entity in renderable_entities {
		transform := kec.get_component(
			s.registry,
			entity,
			comp.Transform_Component,
		)
		shape := kec.get_component(s.registry, entity, comp.Shape_Component)

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
make_behavior_context :: proc(dt: f64) -> comp.Behavior_Context {
	return comp.Behavior_Context {
		dt = dt,
		input = {
			is_key_down = input.is_key_down,
			is_key_up = input.is_key_up,
			is_key_pressed = input.is_key_pressed,
			is_key_released = input.is_key_released,
		},
	}
}
