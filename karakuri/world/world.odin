package karakuri_world

STARTING_CAPACITY :: 1000

import "core:container/queue"
import "base:intrinsics"
import "ktimer:timer"
import v2 "kmath:vector2"
import "../components"
import "../asset_store"

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
		sync_add_entity(&world, &entity)
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

	for &entity in world.entities {
		if behavior, ok := entity.behavior.?; ok {
			free(behavior)
		}
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
find_with_tag :: proc(world: World, tag: string) -> Maybe(^Entity) {
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

// Updates the world
update :: proc(
	world: ^World,
	delta_time: f64,
	timer_info: ^timer.Timer_Info,
	finished_timers: map[uint]struct {},
) {
	// Sync destroy and remove entities
	for &token in world.entities_to_remove {
		entity := &world.entities[token.id]
		if entity.generation_id != token.generation_id {
			continue
		}

		destroy_entity(world, timer_info, entity)
		sync_remove_entity(world, entity)
	}
	clear(&world.entities_to_remove)

	// Sync add and start entities
	entities_to_start := make([dynamic]Token, 0, len(world.entities_to_add))
	defer delete(entities_to_start)

	for &payload in world.entities_to_add {
		token := sync_add_entity(world, &payload)

		append(&entities_to_start, token)
	}
	clear(&world.entities_to_add)

	for &token in entities_to_start {
		entity := &world.entities[token.id]

		start_entity(world, timer_info, entity)
	}

	// Update entities
	update_entities(world, delta_time, timer_info, finished_timers)

	// Run systems
	collision_system(world.entities[:], world, delta_time, timer_info)
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

	on_destroy, on_destroy_ok := behavior.on_destroy.?
	if !on_destroy_ok {
		return
	}

	on_destroy(make_behavior_context(entity, 0, world, timer_info))
}

@(private = "file")
sync_add_entity :: proc(
	world: ^World,
	entity_payload: ^Entity_Payload,
) -> Token {
	if sprite, ok := &entity_payload.sprite.?; ok {
		sprite.sprite.texture = asset_store.get_texture(sprite.sprite_name)
	}

	if box_collider, ok := &entity_payload.box_collider.?;
	   ok && box_collider.size.x == 0 && box_collider.size.y == 0 {
		if sprite, sprite_ok := &entity_payload.sprite.?; sprite_ok {
			box_collider.size =
				sprite.sprite.clip_size.? or_else v2.Vector2 {
					f64(sprite.sprite.texture.width),
					f64(sprite.sprite.texture.height),
				}
		}
	}

	if transform, ok := &entity_payload.transform.?; ok {
		if transform.scale.x == 0 && transform.scale.y == 0 {
			transform.scale = v2.Unit
		}
	} else {
		entity_payload.transform = components.DEFAULT_TRANSFORM_COMPONENT
	}

	new_entity := Entity {
		tag          = entity_payload.tag,
		behavior     = entity_payload.behavior,
		transform    = entity_payload.transform.? or_else components.DEFAULT_TRANSFORM_COMPONENT,
		shape        = entity_payload.shape,
		sprite       = entity_payload.sprite,
		box_collider = entity_payload.box_collider,
	}

	token, token_ok := queue.pop_back_safe(&world.free_tokens)
	if token_ok {
		token.generation_id += 1
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
	}

	queue.append(&world.free_tokens, entity.token)

	// Replace the entity with an empty one
	world.entities[entity.token.id] = Entity {
		token = {generation_id = -1},
	}
}

@(private = "file")
update_entities :: proc(
	world: ^World,
	delta_time: f64,
	timer_info: ^timer.Timer_Info,
	finished_timers: map[uint]struct {},
) {
	for &entity in world.entities {
		behavior, ok := entity.behavior.?
		if !ok {
			continue
		}

		if len(finished_timers) != 0 {
			on_timer, on_timer_ok := behavior.on_timer.?
			if on_timer_ok {
				on_timer(
					make_behavior_context(
						&entity,
						delta_time,
						world,
						timer_info,
					),
					finished_timers,
				)
			}
		}

		on_update, on_update_ok := behavior.on_update.?
		if on_update_ok {
			on_update(
				make_behavior_context(&entity, delta_time, world, timer_info),
			)
		}
	}
}

@(private)
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

