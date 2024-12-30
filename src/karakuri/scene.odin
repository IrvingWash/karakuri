package karakuri

import "../kec"
import ren "../kwindow/renderer"
import fm "../kwindow/fps_manager"
import c "./components"

Scene :: struct {
	registry:			kec.Registry,
	entities_to_add:	[dynamic]c.Component_Bundle,
	entities_to_remove: [dynamic]kec.Entity,
}

new_scene :: proc(initial_entities: [dynamic]c.Component_Bundle) -> Scene {
	scene := Scene {
		registry		   = kec.new_registry(),
		entities_to_add    = make([dynamic]c.Component_Bundle),
		entities_to_remove = make([dynamic]kec.Entity),
	}

	for bundle in initial_entities {
		sync_add_entity(bundle, &scene.registry)
	}

	delete(initial_entities)

	return scene
}

destroy_scene :: proc(s: Scene) {
	kec.destroy_registry(s.registry)
	delete(s.entities_to_add)
	delete(s.entities_to_remove)
}

update_scene :: proc(s: ^Scene, ri: ^ren.Renderer_Info) {
	sync_with_registry(s)
	update_entities(s)
	render_entities(s, ri)
}

add_entity :: proc(bundle: c.Component_Bundle, s: ^Scene) {
	append(&s.entities_to_add, bundle)
}

remove_entity :: proc(entity: kec.Entity, s: ^Scene) {
	append(&s.entities_to_remove, entity)
}

@(private = "file")
sync_with_registry :: proc(s: ^Scene) {
	// TODO: Call on_start and on_destroy for entities

	for entity in s.entities_to_remove {
		sync_remove_entity(entity, &s.registry)
	}

	clear(&s.entities_to_remove)

	for bundle in s.entities_to_add {
		sync_add_entity(bundle, &s.registry)
	}

	clear(&s.entities_to_add)
}

@(private = "file")
sync_add_entity :: proc(bundle: c.Component_Bundle, r: ^kec.Registry) {
	entity := kec.create_entity(r)

	if transform, ok := bundle.transform.?; ok {
		kec.add_component(r, entity, transform)
	} else {
		kec.add_component(r, entity, c.new_transform_component())
	}

	if shape, ok := bundle.shape.?; ok {
		kec.add_component(r, entity, shape)
	}

	if behavior, ok := bundle.behavior.?; ok {
		kec.add_component(r, entity, behavior)
	}
}

@(private = "file")
sync_remove_entity :: proc(entity: kec.Entity, r: ^kec.Registry) {
	kec.destroy_entity(r, entity)
}

@(private = "file")
update_entities :: proc(s: ^Scene) {
	updatable := kec.start_query()
	kec.query_with(c.Behavior_Component, &updatable, s.registry)
	updatable_entities := kec.submit_query(updatable, s.registry)
	defer delete(updatable_entities)

	behavior_context := c.Behavior_Context{
		dt = fm.get_delta_time(),
	}

	for entity in updatable_entities {
		behavior := kec.get_component(s.registry, entity, c.Behavior_Component)

		if on_update, ok := behavior.on_update.?; ok {
			behavior_context.entity = entity

			on_update(behavior_context)
		}
	}
}

@(private = "file")
render_entities :: proc(s: ^Scene, ri: ^ren.Renderer_Info) {
	renderable := kec.start_query()
	kec.query_with(c.Transform_Component, &renderable, s.registry)
	kec.query_with(c.Shape_Component, &renderable, s.registry)
	renderable_entities := kec.submit_query(renderable, s.registry)
	defer delete(renderable_entities)

	ren.start_drawing(ri)

	for entity in renderable_entities {
		transform := kec.get_component(
			s.registry,
			entity,
			c.Transform_Component,
		)
		shape := kec.get_component(s.registry, entity, c.Shape_Component)

		ren.draw_rectangle(
			ri = ri^,
			position = transform.position,
			width = shape.width,
			height = shape.height,
			scale = transform.scale,
			rotation = transform.rotation,
			color = shape.color,
		)
	}

	ren.finish_drawing()
}
