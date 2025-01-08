package karakuri_world

STARTING_CAPACITY :: 1000

import "core:container/queue"
import "../entity"
import "base:intrinsics"

// Represents a world of entities
World :: struct {
	entities:    [dynamic]entity.Entity,
	free_tokens: queue.Queue(entity.Token),
}

// Creates the world
new :: proc() -> World {
	free_tokens: queue.Queue(entity.Token)
	queue.init(&free_tokens, STARTING_CAPACITY)

	return {
		entities = make([dynamic]entity.Entity, 0, STARTING_CAPACITY),
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
add_entity :: proc(world: ^World, entity_payload: entity.Entity_Payload) {
	new_entity := entity.Entity {
		data = entity_payload, // TODO: Handle defaults
	}

	token, token_ok := queue.pop_back_safe(&world.free_tokens)
	if token_ok {
		new_entity.token = token

		world.entities[token.id] = new_entity

		return
	}

	new_entity.token = entity.Token {
		generation_id = 0,
		id            = len(world.entities),
	}

	append(&world.entities, new_entity)
}

// Removes an entity from the world
remove_entity :: proc(world: ^World, token: entity.Token) {
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
is_alive :: proc(world: World, token: entity.Token) -> bool {
	return world.entities[token.id].generation_id == token.generation_id
}

// Returns the first entity with the given tag
find_with_tag :: proc(world: ^World, tag: string) -> Maybe(^entity.Entity) {
	for &e in world.entities {
		if e.tag != tag {
			continue
		}

		return &e
	}

	return nil
}

// Returns the behavior of the entity
get_behavior :: proc(
	e: entity.Entity,
	$Behavior: typeid,
) -> Maybe(^Behavior) where intrinsics.type_is_subtype_of(
		Behavior,
		entity.Behavior,
	) {
	behavior, ok := e.behavior.?
	if !ok {
		return nil
	}

	return cast(^Behavior)behavior
}

