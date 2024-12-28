package kec

@(private = "file")
Component_Array :: [dynamic]rawptr

Registry :: struct {
	component_pools: map[typeid]Component_Pool,
	next_entity:     Entity,
}

Component_Pool :: struct {
	component_array: ^Component_Array,
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

	append(cast(^[dynamic]C)r.component_pools[C].component_array, component)
}

get_component :: proc(r: Registry, entity: Entity, $C: typeid) -> ^C {
	if !(C in r.component_pools) {
		return nil
	}

	return cast(^C)&r.component_pools[C].component_array[entity]
}

@(private)
new_component_pool :: proc() -> Component_Pool {
	component_array := new(Component_Array)
	component_array^ = make(Component_Array)

	return Component_Pool{component_array = component_array}
}

@(private)
destroy_component_pool :: proc(cp: Component_Pool) {
	delete(cp.component_array^)
	free(cp.component_array)
}

@(private)
register_component :: proc(r: ^Registry, $C: typeid) {
	r.component_pools[C] = new_component_pool()
}
