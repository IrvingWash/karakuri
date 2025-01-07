package entity

import "base:intrinsics"

DEFAULT_ENTITY_CAPACITY :: 1000

next_id: uint
entities: [dynamic]^Entity

init_manager :: proc() {
	entities = make([dynamic]^Entity, 0, DEFAULT_ENTITY_CAPACITY)
}

deinit_manager :: proc() {
	delete(entities)
}

@(require_results)
create_entity :: proc(entity: ^Entity) -> uint {
	defer next_id += 1

	entity.id = next_id

	append(&entities, entity)

	return next_id
}

destroy_entity :: proc(id: uint) {
	slot := -1

	for entity, index in entities {
		if (entity.id != id) {
			continue
		}

		slot = index
	}

	if slot == -1 {
		return
	}

	unordered_remove(&entities, slot)
}

get_entity :: proc {
	get_entity_with_tag,
	get_entity_subtype_with_tag,
	get_entity_with_id,
	get_entity_subtype_with_id,
}

@(private = "file")
get_entity_with_tag :: proc(tag: string) -> Maybe(^Entity) {
	for entity in entities {
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
	tag: string,
	$Subtype: typeid,
) -> Maybe(^Entity) where intrinsics.type_is_subtype_of(Entity) {
	entity, ok := get_entity_with_tag(tag).?
	if !ok {
		return nil
	}

	return cast(^Subtype)entity
}

@(private = "file")
get_entity_with_id :: proc(id: uint) -> Maybe(^Entity) {
	for entity in entities {
		if entity.id != id {
			continue
		}

		return entity
	}

	return nil
}

@(private = "file")
get_entity_subtype_with_id :: proc(
	id: uint,
	$Subtype: typeid,
) -> Maybe(^Subtype) where intrinsics.type_is_subtype_of(Subtype, Entity) {
	entity, ok := get_entity_with_id(id).?
	if !ok {
		return nil
	}

	return cast(^Subtype)entity
}

