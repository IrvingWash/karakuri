package karakuri_world_tests

import "core:testing"
import kw "karakuri:world"
import kc "karakuri:components"
import ke "karakuri:entity"
import v2 "kmath:vector2"

@(test)
test_add_entity :: proc(t: ^testing.T) {
	using testing

	world := kw.init_world()
	defer kw.deinit_world(&world)

	kw.add_entity(&world, make_sonic())
	kw.add_entity(&world, make_tails())

	expect_value(t, len(world.entities), 2)

	// Sonic tests
	{
		sonic, sonic_ok := kw.find_with_tag(&world, "Sonic").?
		expect(t, sonic_ok)
		expect_value(t, sonic.transform.position, v2.Vector2{100, 100})
		sonic_behavior, sonic_behavior_ok := kw.get_behavior(sonic^, Sonic).?
		expect(t, sonic_behavior_ok)
		expect_value(t, sonic_behavior.speed, 10)
	}

	// Tails tests
	{
		tails, tails_ok := kw.find_with_tag(&world, "Tails").?
		expect(t, tails_ok)
		expect_value(t, tails.transform.position, v2.Vector2{90, 90})
		tails_behavior, tails_behavior_ok := kw.get_behavior(tails^, Tails).?
		expect(t, tails_behavior_ok)
		expect_value(
			t,
			tails_behavior.get_max_flight_duration(tails_behavior^),
			9,
		)

		tails.transform.position.x += 1
		expect_value(t, tails.transform.position, v2.Vector2{91, 90})
	}
}

@(test)
test_remove_entity :: proc(t: ^testing.T) {
	using testing

	world := kw.init_world()
	defer kw.deinit_world(&world)

	kw.add_entity(&world, make_sonic())
	kw.add_entity(&world, make_tails())

	sonic := kw.find_with_tag(&world, "Sonic").?

	kw.remove_entity(&world, sonic.token)

	expect_value(t, world.free_tokens.len, 1)

	kw.add_entity(&world, make_knuckles())

	expect_value(t, len(world.entities), 2)
	expect_value(t, world.free_tokens.len, 0)
}

@(private = "file")
Sonic :: struct {
	using behavior: ke.Behavior,
	speed:          f64,
}
@(private = "file")
make_sonic :: proc() -> ke.Entity_Payload {
	sonic_behavior := new(Sonic)
	sonic_behavior^ = Sonic {
		speed = 10,
	}

	return ke.Entity_Payload {
		tag = "Sonic",
		transform = kc.Transform_Component {
			position = {100, 100},
			scale = {1, 1},
		},
		behavior = sonic_behavior,
	}
}

@(private = "file")
Tails :: struct {
	using behavior:          ke.Behavior,
	max_flight_duration:     f64,
	get_max_flight_duration: proc(tails: Tails) -> f64,
}
@(private = "file")
make_tails :: proc() -> ke.Entity_Payload {
	tails_behavior := new(Tails)
	tails_behavior^ = Tails {
		max_flight_duration = 9,
		get_max_flight_duration = proc(tails: Tails) -> f64 {
			return tails.max_flight_duration
		},
	}

	return ke.Entity_Payload {
		tag = "Tails",
		transform = kc.Transform_Component {
			position = {90, 90},
			scale = {9, 9},
		},
		behavior = tails_behavior,
	}
}

@(private = "file")
Knuckles :: struct {
	using behavior: ke.Behavior,
	damage:         f64,
}
@(private = "file")
make_knuckles :: proc() -> ke.Entity_Payload {
	knuckles_behavior := new(Knuckles)
	knuckles_behavior^ = Knuckles {
		damage = 8,
	}

	return ke.Entity_Payload {
		tag = "Knuckles",
		transform = kc.Transform_Component {
			position = {80, 80},
			scale = {8, 8},
		},
		behavior = knuckles_behavior,
	}
}

