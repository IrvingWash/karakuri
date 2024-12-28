package registry_tests

import "../../karakuri/kec"
import "core:testing"

HP :: struct {
	value: int,
}

Inventory :: struct {
	rings: uint,
}

@(test)
test_create_entity :: proc(t: ^testing.T) {
	using testing
	using kec

	r := new_registry()

	sonic := create_entity(&r)
	tails := create_entity(&r)
	knuckles := create_entity(&r)

	expect(t, sonic == 0)
	expect(t, tails == 1)
	expect(t, knuckles == 2)

	expect(t, r.next_entity == 3)
}

@(test)
test_add_component :: proc(t: ^testing.T) {
	using testing
	using kec

	r := new_registry()
	defer destroy_registry(r)

	sonic := create_entity(&r)
	tails := create_entity(&r)

	add_component(&r, sonic, HP{100})
	add_component(&r, sonic, Inventory{3})

	expect(t, len(r.component_pools) == 2)
	expect(t, len(r.component_pools[HP].component_array) == 1)
	expect(t, len(r.component_pools[Inventory].component_array) == 1)

	add_component(&r, tails, Inventory{50})

	expect(t, len(r.component_pools) == 2)
	expect(t, len(r.component_pools[HP].component_array) == 1)
	expect(t, len(r.component_pools[Inventory].component_array) == 2)
}

@(test)
test_get_component :: proc(t: ^testing.T) {
	using testing
	using kec

	r := new_registry()
	defer destroy_registry(r)

	sonic := create_entity(&r)
	tails := create_entity(&r)

	add_component(&r, sonic, HP{100})
	add_component(&r, sonic, Inventory{3})
	add_component(&r, tails, Inventory{50})

	sonic_hp := get_component(r, sonic, HP)
	sonic_hp.value += 1
	expect(t, get_component(r, sonic, HP).value == 101)

	sonic_inventory := get_component(r, sonic, Inventory)
	expect(t, sonic_inventory.rings == 3)

	tails_hp := get_component(r, tails, HP)
	expect(t, tails_hp == nil)

	tails_inventory := get_component(r, tails, Inventory)
	expect(t, tails_inventory.rings == 50)
}

test_remove_entity :: proc(t: ^testing.T) {
	using testing
	using kec

	r := new_registry()
	defer destroy_registry(r)

	sonic := create_entity(&r)
	tails := create_entity(&r)
	knuckles := create_entity(&r)

	add_component(&r, sonic, HP{300})
	add_component(&r, sonic, Inventory{30})
	add_component(&r, tails, HP{500})
	add_component(&r, tails, Inventory{50})
	add_component(&r, knuckles, HP{700})
	add_component(&r, tails, Inventory{70})

	remove_entity(&r, tails)

	expect(t, len(r.component_pools[HP].free_slots.data) == 1)

	expect(t, get_component(r, sonic, HP).value == 300)
	expect(t, get_component(r, sonic, Inventory).rings == 30)
	expect(t, get_component(r, knuckles, HP).value == 700)
	expect(t, get_component(r, knuckles, Inventory).rings == 70)
	expect(t, get_component(r, tails, HP) == nil)
	expect(t, get_component(r, tails, Inventory) == nil)

	amy := create_entity(&r)
	add_component(&r, amy, HP{900})
	add_component(&r, amy, Inventory{90})

	expect(t, get_component(r, amy, HP).value == 900)
	expect(t, get_component(r, amy, Inventory).rings == 90)
	expect(t, len(r.component_pools[HP].free_slots.data) == 0)
}
