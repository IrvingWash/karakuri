package kec_tests

import "../../src/kec"
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

	destroy_entity(&r, tails)

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

@(test)
test_query :: proc(t: ^testing.T) {
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

	query_1 := query_start()
	query_with(HP, &query_1, r)
	with_hp := query_submit(query_1, r)
	defer delete(with_hp)

	expect(t, len(with_hp) == 3)

	query_2 := query_start()
	query_with(HP, &query_2, r)
	query_with(Inventory, &query_2, r)
	with_hp_and_inventory := query_submit(query_2, r)
	defer delete(with_hp_and_inventory)

	expect(t, len(with_hp_and_inventory) == 2)

	query_3 := query_start()
	query_with(Inventory, &query_3, r)
	with_inventory := query_submit(query_3, r)
	defer delete(with_inventory)

	expect(t, len(with_inventory) == 2)

	destroy_entity(&r, tails)

	query_4 := query_start()
	query_with(HP, &query_4, r)
	with_hp_after_destroying := query_submit(query_4, r)
	defer delete(with_hp_after_destroying)

	expect(t, len(with_hp_after_destroying) == 2)
}

@(test)
test_get_component_from_query :: proc(t: ^testing.T) {
	using testing
	using kec

	Vector2 :: [2]f64

	Shape :: struct {
		size:  Vector2,
		color: [4]f64,
	}

	Transform :: struct {
		position: Vector2,
		scale:    Vector2,
		rotation: f64,
	}

	r := new_registry()
	defer destroy_registry(r)

	sonic := kec.create_entity(&r)
	tails := kec.create_entity(&r)

	add_component(
		&r,
		sonic,
		Transform{position = {300, 300}, scale = {3, 3}, rotation = 3},
	)
	add_component(&r, sonic, Shape{size = {30, 30}, color = {3, 3, 3, 3}})
	add_component(
		&r,
		tails,
		Transform{position = {500, 500}, scale = {5, 5}, rotation = 5},
	)
	add_component(&r, tails, Shape{size = {50, 50}, color = {5, 5, 5, 5}})

	transforms_and_shapes_query := kec.query_start()
	kec.query_with(Transform, &transforms_and_shapes_query, r)
	kec.query_with(Shape, &transforms_and_shapes_query, r)
	entities_with_transforms_and_shapes := kec.query_submit(
		transforms_and_shapes_query,
		r,
	)
	defer delete(entities_with_transforms_and_shapes)

	expect(t, len(entities_with_transforms_and_shapes) == 2)

	sonic_from_query :=
		entities_with_transforms_and_shapes[0] == sonic ? entities_with_transforms_and_shapes[0] : entities_with_transforms_and_shapes[1]
	tails_from_query :=
		entities_with_transforms_and_shapes[0] == tails ? entities_with_transforms_and_shapes[0] : entities_with_transforms_and_shapes[1]

	sonic_transform := kec.get_component(r, sonic_from_query, Transform)
	tails_transform := kec.get_component(r, tails_from_query, Transform)

	expect(t, sonic_transform.position == Vector2{300, 300})
	expect(t, tails_transform.position == Vector2{500, 500})

	sonic_shape := kec.get_component(r, sonic_from_query, Shape)
	tails_shape := kec.get_component(r, tails_from_query, Shape)

	expect(t, sonic_shape.size == Vector2{30, 30})
	expect(t, tails_shape.size == Vector2{50, 50})

	// After destroy
	kec.destroy_entity(&r, tails)

	transforms_and_shapes_query_2 := kec.query_start()
	kec.query_with(Transform, &transforms_and_shapes_query_2, r)
	kec.query_with(Shape, &transforms_and_shapes_query_2, r)
	entities_with_transforms_and_shapes_2 := kec.query_submit(
		transforms_and_shapes_query_2,
		r,
	)
	defer delete(entities_with_transforms_and_shapes_2)

	expect(t, len(entities_with_transforms_and_shapes_2) == 1)

	sonic_from_query_2 := entities_with_transforms_and_shapes_2[0]

	sonic_transform_2 := kec.get_component(r, sonic_from_query_2, Transform)
	sonic_shape_2 := kec.get_component(r, sonic_from_query_2, Shape)
	expect(t, sonic_transform_2.position == Vector2{300, 300})
	expect(t, sonic_shape_2.size == Vector2{30, 30})

	tails_transform_2 := kec.get_component(r, tails, Transform)
	tails_shape_2 := kec.get_component(r, tails, Shape)
	expect(t, tails_transform_2 == nil)
	expect(t, tails_shape_2 == nil)
}
