package entity_manager

import "base:intrinsics"

DEFAULT_ENTITY_CAPACITY :: 1000

Entity_Manager_Info :: struct {
	next_id:  uint,
	entities: [dynamic]^Entity,
}

init :: proc() -> Entity_Manager_Info {
	return {
		next_id = 0,
		entities = make([dynamic]^Entity, 0, DEFAULT_ENTITY_CAPACITY),
	}
}

deinit :: proc(emi: Entity_Manager_Info) {
	for entity in emi.entities {
		delete(entity.tags)

		free(entity)
	}

	delete(emi.entities)
}

@(require_results)
create_entity :: proc(emi: ^Entity_Manager_Info, entity: ^Entity) -> uint {
	defer emi.next_id += 1

	entity.id = emi.next_id

	append(&emi.entities, entity)

	return emi.next_id
}

destroy_entity :: proc(emi: ^Entity_Manager_Info, id: uint) {
	slot := -1

	for entity, index in emi.entities {
		if (entity.id != id) {
			continue
		}

		delete(entity.tags)

		free(entity)

		slot = index
	}

	if slot == -1 {
		return
	}

	unordered_remove(&emi.entities, slot)
}

get_entities :: proc(emi: Entity_Manager_Info) -> []^Entity {
	return emi.entities[:]
}

get_entity :: proc {
	get_entity_with_tag,
	get_entity_subtype_with_tag,
	get_entity_with_id,
	get_entity_subtype_with_id,
}

@(private = "file")
get_entity_with_tag :: proc(
	emi: Entity_Manager_Info,
	tag: string,
) -> Maybe(^Entity) {
	for entity in emi.entities {
		for current_tag in entity.tags {
			if tag != current_tag {
				continue
			}

			return entity
		}
	}

	return nil
}

@(private = "file")
get_entity_subtype_with_tag :: proc(
	emi: Entity_Manager_Info,
	tag: string,
	$Subtype: typeid,
) -> Maybe(^Subtype) where intrinsics.type_is_subtype_of(Subtype, Entity) {
	entity, ok := get_entity_with_tag(emi, tag).?
	if !ok {
		return nil
	}

	return cast(^Subtype)entity
}

@(private = "file")
get_entity_with_id :: proc(
	emi: Entity_Manager_Info,
	id: uint,
) -> Maybe(^Entity) {
	for entity in emi.entities {
		if entity.id != id {
			continue
		}

		return entity
	}

	return nil
}

@(private = "file")
get_entity_subtype_with_id :: proc(
	emi: Entity_Manager_Info,
	id: uint,
	$Subtype: typeid,
) -> Maybe(^Subtype) where intrinsics.type_is_subtype_of(Subtype, Entity) {
	entity, ok := get_entity_with_id(emi, id).?
	if !ok {
		return nil
	}

	return cast(^Subtype)entity
}

