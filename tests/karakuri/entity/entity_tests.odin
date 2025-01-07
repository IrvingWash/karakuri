package entity_tests

import "core:testing"
import "karakuri:entity_manager"

Sonic :: struct {
	using entity: entity_manager.Entity,
	speed:        f64,
}

Tails :: struct {
	using entity: entity_manager.Entity,
	fly_length:   f64,
}

Knuckles :: struct {
	using entity: entity_manager.Entity,
	attack_move:  struct {
		damage: f64,
		count:  uint,
	},
}

@(test)
test_create_entity :: proc(t: ^testing.T) {
	using testing

	emi := entity_manager.init()
	defer entity_manager.deinit(emi)

	sonic, tails, knuckles := create_entities()

	sonic_id := entity_manager.create_entity(&emi, sonic)
	tails_id := entity_manager.create_entity(&emi, tails)
	knuckles_id := entity_manager.create_entity(&emi, knuckles)

	expect(t, sonic_id == 0)
	expect(t, tails_id == 1)
	expect(t, knuckles_id == 2)

	expect(t, len(entity_manager.get_entities(emi)) == 3)
}

@(test)
test_destroy_entity :: proc(t: ^testing.T) {
	using testing

	emi := entity_manager.init()
	defer entity_manager.deinit(emi)

	sonic, tails, knuckles := create_entities()

	sonic_id := entity_manager.create_entity(&emi, sonic)
	tails_id := entity_manager.create_entity(&emi, tails)
	knuckles_id := entity_manager.create_entity(&emi, knuckles)

	entity_manager.destroy_entity(&emi, tails_id)

	entities := entity_manager.get_entities(emi)

	expect(t, len(entities) == 2)
	expect(t, entities[0].id == sonic_id || entities[1].id == sonic_id)
	expect(t, entities[0].id == knuckles_id || entities[1].id == knuckles_id)
}

@(test)
test_get_entity :: proc(t: ^testing.T) {
	using testing

	emi := entity_manager.init()
	defer entity_manager.deinit(emi)

	sonic, tails, knuckles := create_entities()

	sonic_id := entity_manager.create_entity(&emi, sonic)
	tails_id := entity_manager.create_entity(&emi, tails)
	knuckles_id := entity_manager.create_entity(&emi, knuckles)

	if sonic_entity_gotten, ok := entity_manager.get_entity(emi, "player").?;
	   ok {
		expect(t, ok)
		expect(t, sonic_entity_gotten.transform.position == {100, 100})
		expect(t, sonic_entity_gotten.transform.scale == {2, 2})
		expect(
			t,
			sonic_entity_gotten.tags[0] == "sonic" &&
			sonic_entity_gotten.tags[1] == "player",
		)
	}

	if sonic_gotten, ok := entity_manager.get_entity(emi, sonic_id, Sonic).?;
	   ok {
		expect(t, sonic.speed == 10)

		expect(t, ok)
		expect(t, sonic_gotten.transform.position == {100, 100})
		expect(t, sonic_gotten.transform.scale == {2, 2})
		expect(
			t,
			sonic_gotten.tags[0] == "sonic" &&
			sonic_gotten.tags[1] == "player",
		)
	}

	if tails_entity_gotten, ok := entity_manager.get_entity(emi, tails_id).?;
	   ok {
		tails_entity_gotten.transform.position.x += 1

		expect(t, ok)
		expect(t, tails_entity_gotten.transform.position == {201, 200})
		expect(t, tails_entity_gotten.transform.scale == {2, 2})
		expect(
			t,
			tails_entity_gotten.tags[0] == "tails" &&
			tails_entity_gotten.tags[1] == "support",
		)
	}

	if tails_gotten, ok := entity_manager.get_entity(emi, "tails", Tails).?;
	   ok {
		tails_gotten.fly_length -= 1

		expect(t, tails_gotten.fly_length == 7)

		expect(t, ok)
		expect(t, tails_gotten.transform.position == {201, 200})
		expect(t, tails_gotten.transform.scale == {2, 2})
		expect(
			t,
			tails_gotten.tags[0] == "tails" &&
			tails_gotten.tags[1] == "support",
		)
	}

	entity_manager.destroy_entity(&emi, tails_id)

	if knuckles_entity_gotten, ok := entity_manager.get_entity(
		   emi,
		   knuckles_id,
	   ).?; ok {
		expect(t, ok)
		expect(t, knuckles_entity_gotten.transform.position == {300, 300})
		expect(t, knuckles_entity_gotten.transform.scale == {2, 2})
		expect(
			t,
			knuckles_entity_gotten.tags[0] == "knuckles" &&
			knuckles_entity_gotten.tags[1] == "support",
		)
	}

	if knuckles_gotten, ok := entity_manager.get_entity(
		   emi,
		   knuckles_id,
		   Knuckles,
	   ).?; ok {
		expect(t, ok)
		expect(t, knuckles_gotten.transform.position == {300, 300})
		expect(t, knuckles_gotten.transform.scale == {2, 2})
		expect(
			t,
			knuckles_gotten.tags[0] == "knuckles" &&
			knuckles_gotten.tags[1] == "support",
		)
	}

	eggman_id: uint = 7
	_, eggman_ok := entity_manager.get_entity(emi, eggman_id).?

	expect(t, !eggman_ok)
}

create_entities :: proc() -> (^Sonic, ^Tails, ^Knuckles) {
	sonic := new(Sonic)
	sonic^ = Sonic {
		transform = {position = {100, 100}, scale = {2, 2}},
		tags = {"sonic", "player"},
		speed = 10,
	}

	tails := new(Tails)
	tails^ = Tails {
		transform = {position = {200, 200}, scale = {2, 2}},
		tags = {"tails", "support"},
		fly_length = 8,
	}

	knuckles := new(Knuckles)
	knuckles^ = Knuckles {
		transform = {position = {300, 300}, scale = {2, 2}},
		tags = {"knuckles", "support"},
		attack_move = {damage = 6, count = 4},
	}

	return sonic, tails, knuckles
}

