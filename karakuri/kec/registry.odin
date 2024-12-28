package kec

Registry :: struct {
	next_entity: Entity,
}

new_registry :: proc() -> Registry {
	return Registry{}
}

create_entity :: proc(r: ^Registry) -> Entity {
	defer r.next_entity += 1

	return r.next_entity
}
