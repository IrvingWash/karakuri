#+private

package karakuri

import "../kec"

Spawner :: struct {
	entities_to_add:    [dynamic]Component_Bundle,
	entities_to_remove: [dynamic]kec.Entity,
	add_entity:         type_of(add_entity),
	remove_entity:      type_of(remove_entity),
}

new_spawner :: proc() -> Spawner {
	return Spawner {
		entities_to_add = make([dynamic]Component_Bundle),
		entities_to_remove = make([dynamic]kec.Entity),
		add_entity = add_entity,
		remove_entity = remove_entity,
	}
}

destroy_spawner :: proc(spawner: Spawner) {
	delete(spawner.entities_to_add)
	delete(spawner.entities_to_remove)
}

add_entity :: proc(spawner: ^Spawner, bundle: Component_Bundle) {
	append(&spawner.entities_to_add, bundle)
}

remove_entity :: proc(spawner: ^Spawner, entity: kec.Entity) {
	append(&spawner.entities_to_remove, entity)
}
