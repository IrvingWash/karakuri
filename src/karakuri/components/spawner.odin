package components

import "../../kec"

Spawner_Info :: struct {
	entities_to_add:    [dynamic]Component_Bundle,
	entities_to_remove: [dynamic]kec.Entity,
	add_entity:         type_of(spawner_add_entity),
	remove_entity:      type_of(spawner_remove_entity),
}

new_spawner_info :: proc() -> Spawner_Info {
	return Spawner_Info {
		entities_to_add = make([dynamic]Component_Bundle),
		entities_to_remove = make([dynamic]kec.Entity),
		add_entity = spawner_add_entity,
		remove_entity = spawner_remove_entity,
	}
}

destroy_spawner_info :: proc(spawner_info: Spawner_Info) {
	delete(spawner_info.entities_to_add)
	delete(spawner_info.entities_to_remove)
}

spawner_add_entity :: proc(
	spawner_info: ^Spawner_Info,
	bundle: Component_Bundle,
) {
	append(&spawner_info.entities_to_add, bundle)
}

spawner_remove_entity :: proc(
	spawner_info: ^Spawner_Info,
	entity: kec.Entity,
) {
	append(&spawner_info.entities_to_remove, entity)
}

