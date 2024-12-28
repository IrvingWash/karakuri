package kec

import q "core:container/queue"

@(private = "file")
Component_Array :: [dynamic]rawptr

@(private = "file")
EntityToComponentSlotMap :: map[Entity]int

Registry :: struct {
	component_pools: map[typeid]Component_Pool,
	next_entity:     Entity,
}

Component_Pool :: struct {
	component_array: ^Component_Array,
	etcsm:           EntityToComponentSlotMap,
	free_slots:      q.Queue(int),
}

new_registry :: proc() -> Registry {
	return Registry{}
}

destroy_registry :: proc(r: Registry) {
	for _, cp in r.component_pools {
		destroy_component_pool(cp)
	}

	delete(r.component_pools)
}

create_entity :: proc(r: ^Registry) -> Entity {
	defer r.next_entity += 1

	return r.next_entity
}

add_component :: proc(r: ^Registry, entity: Entity, component: $C) {
	if !(C in r.component_pools) {
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
}

get_component :: proc(r: Registry, entity: Entity, $C: typeid) -> ^C {
	if !(C in r.component_pools) {
		return nil
	}

	component_pool := &r.component_pools[C]

	slot, ok := component_pool.etcsm[entity]
	if !ok {
		return nil
	}

	return cast(^C)&component_pool.component_array[slot]
}

@(private)
new_component_pool :: proc() -> Component_Pool {
	component_array := new(Component_Array)
	component_array^ = make(Component_Array)

	return Component_Pool {
		component_array = component_array,
		etcsm = make(EntityToComponentSlotMap),
		free_slots = q.Queue(int){},
	}
}

@(private)
destroy_component_pool :: proc(cp: Component_Pool) {
	delete(cp.component_array^)
	free(cp.component_array)

	delete(cp.etcsm)
}

@(private)
register_component :: proc(r: ^Registry, $C: typeid) {
	r.component_pools[C] = new_component_pool()
}
