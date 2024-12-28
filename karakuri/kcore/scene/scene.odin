package scene

import "../../kec"
import c "../components"

Scene :: struct {
	entities_to_add:    [dynamic]c.Component_Bundle,
	entities_to_remove: [dynamic]kec.Entity,
}

new_scene :: proc() -> Scene {
	return Scene {
		entities_to_add = make([dynamic]c.Component_Bundle),
		entities_to_remove = make([dynamic]kec.Entity),
	}
}

destroy_scene :: proc(s: Scene) {
	delete(s.entities_to_remove)
	delete(s.entities_to_add)
}

add_entity :: proc(s: ^Scene, bundle: c.Component_Bundle) {
	append(&s.entities_to_add, bundle)
}

remove_entity :: proc(s: ^Scene, entity: kec.Entity) {
	append(&s.entities_to_remove, entity)
}

sync :: proc(s: ^Scene, r: ^kec.Registry) {
	sync_remove(s, r)
	sync_add(s, r)
}

@(private = "file")
sync_remove :: proc(s: ^Scene, r: ^kec.Registry) {
	for entity in s.entities_to_remove {
		kec.destroy_entity(r, entity)
	}

	clear(&s.entities_to_remove)
}

@(private = "file")
sync_add :: proc(s: ^Scene, r: ^kec.Registry) {
	for bundle in &s.entities_to_add {
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

	clear(&s.entities_to_add)
}
