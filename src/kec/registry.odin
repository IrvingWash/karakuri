package kec

import q "core:container/queue"
import "core:reflect"

@(private = "file")
Component_Array :: [dynamic]rawptr

@(private = "file")
Entity_To_Component_Slot_Map :: map[Entity]int

Registry :: struct {
	component_pools:   map[typeid]Component_Pool,
	next_entity:       Entity,
	component_ids:     map[typeid]int,
	next_component_id: int,
	entity_signatures: map[Entity]Signature,
}

@(private = "file")
Component_Pool :: struct {
	component_array: ^Component_Array,
	etcsm:           Entity_To_Component_Slot_Map,
	free_slots:      q.Queue(int),
}

new_registry :: proc() -> Registry {
	return Registry{}
}

destroy_registry :: proc(r: Registry) {
	for component_type, &cp in r.component_pools {
		destroy_component_pool(&cp, component_type)
	}

	delete(r.component_pools)
	delete(r.component_ids)
	delete(r.entity_signatures)
}

create_entity :: proc(r: ^Registry) -> Entity {
	defer r.next_entity += 1

	return r.next_entity
}

destroy_entity :: proc(r: ^Registry, entity: Entity) {
	for component_type, &component_pool in r.component_pools {
		slot, ok := component_pool.etcsm[entity]
		if ok {
			if reflect.is_pointer(type_info_of(component_type)) {
				free(component_pool.component_array[slot])
			}

			delete_key(&component_pool.etcsm, entity)

			q.push_back(&component_pool.free_slots, slot)
		}

		delete_key(&r.entity_signatures, entity)
	}
}

add_component :: proc(r: ^Registry, entity: Entity, component: $C) {
	if C not_in r.component_pools {
		register_component(r, C)
	}

	component_pool := &r.component_pools[C]

	slot, slot_ok := q.pop_back_safe(&component_pool.free_slots)
	if slot_ok {
		(cast(^[dynamic]C)component_pool.component_array)[slot] = component
	} else {
		append(cast(^[dynamic]C)component_pool.component_array, component)
		slot = len(r.component_pools[C].component_array) - 1
	}

	etcsm := &component_pool.etcsm
	etcsm[entity] = slot

	entity_sig, entity_sig_ok := &r.entity_signatures[entity]
	if entity_sig_ok {
		entity_sig^ += {r.component_ids[C]}
	} else {
		r.entity_signatures[entity] = Signature{r.component_ids[C]}
	}
}

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

get_component_double_cast :: proc(
	r: Registry,
	entity: Entity,
	$C: typeid,
	$A: typeid,
) -> ^A {
	first := get_component(r, entity, ^C)

	second := cast(^^A)first

	return second^
}

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

@(private)
destroy_component_pool :: proc(cp: ^Component_Pool, component_type: typeid) {
	if reflect.is_pointer(type_info_of(component_type)) {
		for component in cp.component_array {
			free(component)
		}
	}

	delete(cp.component_array^)
	q.destroy(&cp.free_slots)
	free(cp.component_array)

	delete(cp.etcsm)
}

@(private)
register_component :: proc(r: ^Registry, $C: typeid) {
	defer r.next_component_id += 1

	r.component_ids[C] = r.next_component_id

	r.component_pools[C] = new_component_pool()
}

