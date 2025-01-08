package karakuri_world

STARTING_CAPACITY :: 1000

import "core:container/queue"
import "base:intrinsics"
import "ktimer:timer"

// Represents a world of entities
World :: struct {
	entities:           [dynamic]Entity,
	entities_to_add:    [dynamic]Entity_Payload,
	entities_to_remove: [dynamic]Token,
	free_tokens:        queue.Queue(Token),
}

// Creates the world
new :: proc(
	initial_entities: []Entity_Payload,
	timer_info: ^timer.Timer_Info,
) -> World {
	free_tokens: queue.Queue(Token)
	queue.init(&free_tokens, STARTING_CAPACITY)

	world := World {
		entities           = make([dynamic]Entity, 0, STARTING_CAPACITY),
		entities_to_add    = make(
			[dynamic]Entity_Payload,
			0,
			STARTING_CAPACITY,
		),
		entities_to_remove = make([dynamic]Token, 0, STARTING_CAPACITY),
		free_tokens        = free_tokens,
	}

	// Add initial entities
	for &entity in initial_entities {
		sync_add_entity(&world, entity)
	}

	// Start initial entities
	for &entity in world.entities {
		start_entity(&world, timer_info, &entity)
	}

	return world
}

// Destroys the world
destroy :: proc(world: ^World, timer_info: ^timer.Timer_Info) {
	// Destroy all remaining entities
	for &entity in world.entities {
		destroy_entity(world, timer_info, &entity)
	}

	// Cleanup
	delete(world.entities)
	delete(world.entities_to_add)
	delete(world.entities_to_remove)
	queue.destroy(&world.free_tokens)
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

// Safely add an entity to the world
add_entity :: proc(world: ^World, payload: Entity_Payload) {
	append(&world.entities_to_add, payload)
}

// Safely remove an entity from the world
remove_entity :: proc(world: ^World, token: Token) {
	append(&world.entities_to_remove, token)
}

update :: proc(world: ^World, delta_time: f64, timer_info: ^timer.Timer_Info) {
	// Sync destroy and remove entities
	for &token in world.entities_to_remove {
		entity := &world.entities[token.id]
		if entity.generation_id != token.generation_id {
			continue
		}

		sync_remove_entity(world, entity)
		destroy_entity(world, timer_info, entity)
	}
	clear(&world.entities_to_remove)

	// Sync add and start entities
	entities_to_start := make([dynamic]Token, 0, len(world.entities_to_add))
	defer delete(entities_to_start)

	for &payload in world.entities_to_add {
		token := sync_add_entity(world, payload)

		append(&entities_to_start, token)
	}
	clear(&world.entities_to_add)

	for &token in entities_to_start {
		entity := &world.entities[token.id]

		start_entity(world, timer_info, entity)
	}

	// Update entities
	update_entities(world, delta_time, timer_info)
}

@(private = "file")
start_entity :: proc(
	world: ^World,
	timer_info: ^timer.Timer_Info,
	entity: ^Entity,
) {
	behavior, ok := entity.behavior.?
	if !ok {
		return
	}

	on_start, on_start_ok := behavior.on_start.?
	if !on_start_ok {
		return
	}

	on_start(make_behavior_context(entity, 0, world, timer_info))
}

@(private = "file")
destroy_entity :: proc(
	world: ^World,
	timer_info: ^timer.Timer_Info,
	entity: ^Entity,
) {
	behavior, behavior_ok := entity.behavior.?
	if !behavior_ok {
		return
	}

	defer free(behavior)

	on_destroy, on_destroy_ok := behavior.on_destroy.?
	if !on_destroy_ok {
		return
	}

	on_destroy(make_behavior_context(entity, 0, world, timer_info))
}

@(private = "file")
sync_add_entity :: proc(
	world: ^World,
	entity_payload: Entity_Payload,
) -> Token {
	new_entity := Entity {
		data = entity_payload, // TODO: Handle defaults
	}

	token, token_ok := queue.pop_back_safe(&world.free_tokens)
	if token_ok {
		new_entity.token = token

		world.entities[token.id] = new_entity

		return token
	}

	token = Token {
		generation_id = 0,
		id            = len(world.entities),
	}

	new_entity.token = token

	append(&world.entities, new_entity)

	return token
}

@(private = "file")
sync_remove_entity :: proc(world: ^World, entity: ^Entity) {
	if behavior, ok := entity.behavior.?; ok {
		free(behavior)
		entity.behavior = nil
	}

	queue.append(&world.free_tokens, entity.token)

	entity.token.generation_id = -1
}

@(private = "file")
update_entities :: proc(
	world: ^World,
	delta_time: f64,
	timer_info: ^timer.Timer_Info,
) {
	for &entity in world.entities {
		behavior, ok := entity.behavior.?
		if !ok {
			continue
		}

		on_update, on_update_ok := behavior.on_update.?
		if on_update_ok {
			on_update(
				make_behavior_context(&entity, delta_time, world, timer_info),
			)
		}
	}
}

@(private = "file")
make_behavior_context :: proc(
	entity: ^Entity,
	delta_time: f64,
	world: ^World,
	timer_info: ^timer.Timer_Info,
) -> Behavior_Context {
	return Behavior_Context {
		self = entity,
		delta_time = delta_time,
		world = world,
		timer_info = timer_info,
	}
}

