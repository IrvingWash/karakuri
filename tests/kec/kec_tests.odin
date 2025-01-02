package kec_tests

import "../../src/kec"
import "core:testing"

/* 
   HP is a simple component representing an entity's health points.
*/
HP :: struct {
    value: int,
}

/*
   Inventory is a sample component that keeps track of a ring count.
*/
Inventory :: struct {
    rings: uint,
}

/*
   test_create_entity verifies that creating new entities 
   assigns them incremental IDs as expected.
*/
@(test)
test_create_entity :: proc(t: ^testing.T) {
    using testing
    using kec

    r := new_registry()

    sonic := create_entity(&r)
    tails := create_entity(&r)
    knuckles := create_entity(&r)

    expect(t, sonic == 0, "First entity should have ID 0.")
    expect(t, tails == 1, "Second entity should have ID 1.")
    expect(t, knuckles == 2, "Third entity should have ID 2.")

    expect(t, r.next_entity == 3, "Next entity ID should be 3 after creating three entities.")
}

/*
   test_add_component checks that adding components creates
   and updates the underlying component pool data structures.
*/
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

    expect(t, len(r.component_pools) == 2, "There should be 2 different component pools.")
    expect(t, len(r.component_pools[HP].component_array) == 1,
        "HP pool should have 1 entry.")
    expect(t, len(r.component_pools[Inventory].component_array) == 1,
        "Inventory pool should have 1 entry.")

    add_component(&r, tails, Inventory{50})

    expect(t, len(r.component_pools) == 2, "Still 2 different pools total.")
    expect(t, len(r.component_pools[HP].component_array) == 1,
        "HP pool remains with 1 entry.")
    expect(t, len(r.component_pools[Inventory].component_array) == 2,
        "Inventory pool should now have 2 entries.")
}

/*
   test_get_component ensures we can retrieve components properly and
   that retrieving non-existing components returns nil.
*/
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

    // Modify sonic's HP via retrieved pointer
    sonic_hp := get_component(r, sonic, HP)
    sonic_hp.value += 1
    expect(t, get_component(r, sonic, HP).value == 101,
        "HP should have increased to 101.")

    // Retrieve inventory
    sonic_inventory := get_component(r, sonic, Inventory)
    expect(t, sonic_inventory.rings == 3, "Sonic's inventory should have 3 rings.")

    // Tails does not have HP
    tails_hp := get_component(r, tails, HP)
    expect(t, tails_hp == nil, "Tails should have no HP component.")

    // Tails has inventory
    tails_inventory := get_component(r, tails, Inventory)
    expect(t, tails_inventory.rings == 50, "Tails' inventory should have 50 rings.")
}

/*
   test_remove_entity checks that destroying an entity cleans up
   its components in the relevant component pools.
*/
@(test)
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
    // Minor bug fix: This line used to add Inventory to tails again. Let's fix it logically:
    add_component(&r, knuckles, Inventory{70})

    destroy_entity(&r, tails)

    // The HP pool for tails should become free
    expect(t, len(r.component_pools[HP].free_slots.data) == 1,
        "One free slot in HP pool after removing tails.")

    expect(t, get_component(r, sonic, HP).value == 300, "Sonic's HP remains 300.")
    expect(t, get_component(r, sonic, Inventory).rings == 30, "Sonic's inventory rings remain 30.")
    expect(t, get_component(r, knuckles, HP).value == 700, "Knuckles' HP remains 700.")
    expect(t, get_component(r, knuckles, Inventory).rings == 70, "Knuckles' inventory rings remain 70.")
    expect(t, get_component(r, tails, HP) == nil, "Tails' HP is removed.")
    expect(t, get_component(r, tails, Inventory) == nil, "Tails' inventory is removed.")

    amy := create_entity(&r)
    add_component(&r, amy, HP{900})
    add_component(&r, amy, Inventory{90})

    expect(t, get_component(r, amy, HP).value == 900, "Amy's HP is 900.")
    expect(t, get_component(r, amy, Inventory).rings == 90, "Amy's inventory rings is 90.")
    expect(t, len(r.component_pools[HP].free_slots.data) == 0,
        "No free slots remain after reusing the old tail's slot for Amy's HP.")
}

/*
   test_query checks we can retrieve entity lists filtered by specific components.
*/
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

    // Query 1: Entities with HP
    query_1 := query_start()
    query_with(HP, &query_1, r)
    with_hp := query_submit(query_1, r)
    defer delete(with_hp)

    expect(t, len(with_hp) == 3, "Three entities (sonic, tails, knuckles) have HP.")

    // Query 2: Entities with HP AND Inventory
    query_2 := query_start()
    query_with(HP, &query_2, r)
    query_with(Inventory, &query_2, r)
    with_hp_and_inventory := query_submit(query_2, r)
    defer delete(with_hp_and_inventory)

    expect(t, len(with_hp_and_inventory) == 2, "Only sonic and tails have both HP and Inventory.")

    // Query 3: Entities with Inventory
    query_3 := query_start()
    query_with(Inventory, &query_3, r)
    with_inventory := query_submit(query_3, r)
    defer delete(with_inventory)

    expect(t, len(with_inventory) == 2, "Sonic and tails have Inventory.")

    // Destroy tails
    destroy_entity(&r, tails)

    // Query 4: Entities with HP after destroying tails
    query_4 := query_start()
    query_with(HP, &query_4, r)
    with_hp_after_destroying := query_submit(query_4, r)
    defer delete(with_hp_after_destroying)

    expect(t, len(with_hp_after_destroying) == 2, "Tails is gone, so sonic and knuckles remain.")
}

/*
   test_get_component_from_query ensures we can retrieve the correct
   Transform and Shape from a query, and that destroying an entity
   invalidates its components.
/*
   test_get_component_from_query ensures we can retrieve the correct
   Transform and Shape from a query, and that destroying an entity
   invalidates its components.
*/
@(test)
test_get_component_from_query :: proc(t: ^testing.T) {
    using testing
    using kec

    // We'll define a local Vector2 type (an array of 2 f64s).
    Vector2 :: [2]f64

    // Define a local Shape component with size and color data.
    Shape :: struct {
        size:  Vector2,
        color: [4]f64,
    }

    // Define a local Transform component with position, scale, and rotation data.
    Transform :: struct {
        position: Vector2,
        scale:    Vector2,
        rotation: f64,
    }

    // 1. Create a new registry and ensure we clean it up afterward.
    r := new_registry()
    defer destroy_registry(r)

    // 2. Create two entities: sonic and tails.
    sonic   := kec.create_entity(&r)
    tails   := kec.create_entity(&r)

    // 3. Give both sonic and tails a Transform and a Shape component, with distinct values.
    add_component(&r, sonic,
        Transform{
            position = {300, 300},
            scale    = {3, 3},
            rotation = 3,
        }
    )
    add_component(&r, sonic,
        Shape{
            size  = {30, 30},
            color = {3, 3, 3, 3},
        }
    )

    add_component(&r, tails,
        Transform{
            position = {500, 500},
            scale    = {5, 5},
            rotation = 5,
        }
    )
    add_component(&r, tails,
        Shape{
            size  = {50, 50},
            color = {5, 5, 5, 5},
        }
    )

    //
    // 4. Query for all entities that have both Transform and Shape components.
    //
    transforms_and_shapes_query := kec.query_start()
    kec.query_with(Transform, &transforms_and_shapes_query, r)
    kec.query_with(Shape, &transforms_and_shapes_query, r)

    entities_with_transforms_and_shapes := kec.query_submit(transforms_and_shapes_query, r)
    defer delete(entities_with_transforms_and_shapes)

    expect(t, len(entities_with_transforms_and_shapes) == 2,
        "Sonic and Tails both should have Transform + Shape."
    )

    // Identify which array index corresponds to sonic or tails.
    sonic_from_query := if entities_with_transforms_and_shapes[0] == sonic {
        entities_with_transforms_and_shapes[0]
    } else {
        entities_with_transforms_and_shapes[1]
    }
    tails_from_query := if entities_with_transforms_and_shapes[0] == tails {
        entities_with_transforms_and_shapes[0]
    } else {
        entities_with_transforms_and_shapes[1]
    }

    // Retrieve their Transform components.
    sonic_transform := kec.get_component(r, sonic_from_query, Transform)
    tails_transform := kec.get_component(r, tails_from_query, Transform)

    expect(t, sonic_transform.position == Vector2{300, 300},
        "Sonic's transform has position (300,300)."
    )
    expect(t, tails_transform.position == Vector2{500, 500},
        "Tails' transform has position (500,500)."
    )

    // Retrieve their Shape components.
    sonic_shape := kec.get_component(r, sonic_from_query, Shape)
    tails_shape := kec.get_component(r, tails_from_query, Shape)

    expect(t, sonic_shape.size == Vector2{30, 30},
        "Sonic's shape size is (30,30)."
    )
    expect(t, tails_shape.size == Vector2{50, 50},
        "Tails' shape size is (50,50)."
    )

    //
    // 5. Destroy sonic to ensure his components become invalidated, leaving tails alone.
    //
    kec.destroy_entity(&r, sonic)

    // Now query again for all entities that have both Transform and Shape.
    transforms_and_shapes_query_2 := kec.query_start()
    kec.query_with(Transform, &transforms_and_shapes_query_2, r)
    kec.query_with(Shape, &transforms_and_shapes_query_2, r)

    entities_with_transforms_and_shapes_2 := kec.query_submit(transforms_and_shapes_query_2, r)
    defer delete(entities_with_transforms_and_shapes_2)

    expect(t, len(entities_with_transforms_and_shapes_2) == 1,
        "Only tails remains with Transform + Shape after sonic is destroyed."
    )

    // Make sure sonic's components are now invalid, and tails is unaffected.
    sonic_transform_2 := kec.get_component(r, sonic, Transform)
    sonic_shape_2     := kec.get_component(r, sonic, Shape)
    expect(t, sonic_transform_2 == nil,
        "Sonic's transform is invalid after destruction."
    )
    expect(t, sonic_shape_2 == nil,
        "Sonic's shape is invalid after destruction."
    )

    // The one remaining entity is tails. Retrieve its transform & shape from the query results.
    tails_from_query_2 := entities_with_transforms_and_shapes_2[0]
    tails_transform_2  := kec.get_component(r, tails_from_query_2, Transform)
    tails_shape_2      := kec.get_component(r, tails_from_query_2, Shape)

    expect(t, tails_transform_2.position == Vector2{500, 500},
        "Tails' transform is unaffected, still (500, 500)."
    )
    expect(t, tails_shape_2.size == Vector2{50, 50},
        "Tails' shape is unaffected, still (50,50)."
    )
}
