package kec

import q "core:container/queue"

@(private = "file")
Component_Array :: [dynamic]rawptr

@(private = "file")
Entity_To_Component_Slot_Map :: map[Entity]int

// Contains all the components associated to entities that have been attached to entities
// as well as some additional info.
// Initialized with `new_registry`
Registry :: struct {
	component_pools:   map[typeid]Component_Pool,
	next_entity:       Entity,
	component_ids:     map[typeid]int,
	next_component_id: int,
	entity_signatures: map[Entity]Signature,
}

// A pool of components of the same time with some additional info
// Initialized with `new_component_pool`
@(private = "file")
Component_Pool :: struct {
	component_array: ^Component_Array,
	etcsm:           Entity_To_Component_Slot_Map,
	free_slots:      q.Queue(int),
}

// Initialized a new registry
new_registry :: proc() -> Registry {
	return Registry{}
}

// Cleans up a registry
destroy_registry :: proc(r: Registry) {
	for _, &cp in r.component_pools {
		destroy_component_pool(&cp)
	}

	delete(r.component_pools)
	delete(r.component_ids)
	delete(r.entity_signatures)
}

// Creates a new entity to which components can be attached.
// Every entity is unique to a registry.
create_entity :: proc(r: ^Registry) -> Entity {
	defer r.next_entity += 1

	return r.next_entity
}

// Cleans up an entity, removing all the components attached to it.
destroy_entity :: proc(r: ^Registry, entity: Entity) {
	for _, &component_pool in r.component_pools {
		slot, ok := component_pool.etcsm[entity]
		if ok {
			delete_key(&component_pool.etcsm, entity)

			q.push_back(&component_pool.free_slots, slot)
		}

		delete_key(&r.entity_signatures, entity)
	}
}

// Adds a components to an entity.
add_component :: proc(r: ^Registry, entity: Entity, component: $C) {
	if C not_in r.component_pools {
		register_component(r, C)
	}

	component_pool := &r.component_pools[C]

	append(cast(^[dynamic]C)component_pool.component_array, component)

	slot, slot_ok := q.pop_back_safe(&component_pool.free_slots)
	if !slot_ok {
		slot = len(r.component_pools[C].component_array) - 1
	}

	etcsm := &component_pool.etcsm
	etcsm[entity] = slot

	entity_sig, entity_sig_ok := &r.entity_signatures[entity]
	if entity_sig_ok {
		entity_sig^ += {r.component_ids[C]}
	} else {
		r.entity_signatures[entity] = {r.component_ids[C]}
	}
}

// Gets a component attached to an entity.
get_component :: proc(r: Registry, entity: Entity, $C: typeid) -> ^C {
	if C not_in r.component_pools {
		return nil
	}

	component_pool := &r.component_pools[C]

	slot, ok := component_pool.etcsm[entity]
	if !ok {
		return nil
	}

	array := cast(^[dynamic]C)component_pool.component_array

	return &array[slot]
}

// Initializes a new component_pool.
@(private)
new_component_pool :: proc() -> Component_Pool {
	component_array := new(Component_Array)
	component_array^ = make(Component_Array)

	return Component_Pool {
		component_array = component_array,
		etcsm = make(Entity_To_Component_Slot_Map),
		free_slots = q.Queue(int){},
	}
}

// Cleans up a component pool.
@(private)
destroy_component_pool :: proc(cp: ^Component_Pool) {
	delete(cp.component_array^)
	q.destroy(&cp.free_slots)
	free(cp.component_array)

	delete(cp.etcsm)
}

// Registers a new component type.
@(private)
register_component :: proc(r: ^Registry, $C: typeid) {
	defer r.next_component_id += 1

	r.component_ids[C] = r.next_component_id

	r.component_pools[C] = new_component_pool()
}
