package karakuri_world

STARTING_CAPACITY :: 1000

import "core:container/queue"
import "base:intrinsics"

// Represents a world of entities
World :: struct {
	entities:    [dynamic]Entity,
	free_tokens: queue.Queue(Token),
}

// Creates the world
new :: proc() -> World {
	free_tokens: queue.Queue(Token)
	queue.init(&free_tokens, STARTING_CAPACITY)

	return {
		entities = make([dynamic]Entity, 0, STARTING_CAPACITY),
		free_tokens = free_tokens,
	}
}

// Destroys the world
destroy :: proc(world: ^World) {
	for &entity in world.entities {
		if behavior, ok := entity.behavior.?; ok {
			free(behavior)
		}
	}

	delete(world.entities)
	queue.destroy(&world.free_tokens)
}

// Adds a new entity to the world
add_entity :: proc(world: ^World, entity_payload: Entity_Payload) {
	new_entity := Entity {
		data = entity_payload, // TODO: Handle defaults
	}

	token, token_ok := queue.pop_back_safe(&world.free_tokens)
	if token_ok {
		new_entity.token = token

		world.entities[token.id] = new_entity

		return
	}

	new_entity.token = Token {
		generation_id = 0,
		id            = len(world.entities),
	}

	append(&world.entities, new_entity)
}

// Removes an entity from the world
remove_entity :: proc(world: ^World, token: Token) {
	stored_entity := &world.entities[token.id]
	if stored_entity.generation_id != token.generation_id {
		return
	}

	if behavior, ok := stored_entity.behavior.?; ok {
		free(behavior)
		stored_entity.behavior = nil
	}

	queue.append(&world.free_tokens, stored_entity.token)

	stored_entity.token.generation_id = -1
}

// Checks if the given entity is still in the world
is_alive :: proc(world: World, token: Token) -> bool {
	return world.entities[token.id].generation_id == token.generation_id
}

// Efficiently returns the entity with the given token
get_entity :: proc(world: ^World, token: Token) -> Maybe(^Entity) {
	entity := &world.entities[token.id]

	if entity.token.generation_id == -1 {
		return nil
	}

	return entity
}

// Returns the first entity with the given tag
find_with_tag :: proc(world: ^World, tag: string) -> Maybe(^Entity) {
	for &entity in world.entities {
		if entity.tag != tag {
			continue
		}

		return &entity
	}

	return nil
}

// Returns the behavior of the entity
get_behavior :: proc(
	entity: Entity,
	$B: typeid,
) -> Maybe(^B) where intrinsics.type_is_subtype_of(B, Behavior) {
	behavior, ok := entity.behavior.?
	if !ok {
		return nil
	}

	return cast(^B)behavior
}

