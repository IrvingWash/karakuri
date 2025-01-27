package karakuri_world_tests

import "core:testing"
import kw "karakuri:world"
import kc "karakuri:components"
import v2 "kmath:vector2"
import "ktimer:timer"

@(test)
test_add_entity :: proc(t: ^testing.T) {
	using testing

	timer_info := &timer.Timer_Info{}

	world := kw.new({}, timer_info)
	defer kw.destroy(&world, timer_info)

	kw.add_entity(&world, make_sonic())
	kw.add_entity(&world, make_tails())

	kw.update(&world, 0, timer_info, {})

	expect_value(t, len(world.entities), 2)

	// Sonic tests
	{
		sonic, sonic_ok := kw.find_with_tag(world, "Sonic").?
		expect(t, sonic_ok)
		expect_value(t, sonic.transform.position, v2.Vector2{100, 100})
		sonic_behavior, sonic_behavior_ok := kw.get_behavior(sonic^, Sonic).?
		expect(t, sonic_behavior_ok)
		expect_value(t, sonic_behavior.speed, 10)
	}

	// Tails tests
	{
		tails, tails_ok := kw.find_with_tag(world, "Tails").?
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

	timer_info := &timer.Timer_Info{}

	world := kw.new({}, timer_info)
	defer kw.destroy(&world, timer_info)

	kw.add_entity(&world, make_sonic())
	kw.add_entity(&world, make_tails())

	kw.update(&world, 0, timer_info, {})

	sonic := kw.find_with_tag(world, "Sonic").?

	kw.remove_entity(&world, sonic.token)

	kw.update(&world, 0, timer_info, {})

	expect_value(t, world.free_tokens.len, 1)

	kw.add_entity(&world, make_knuckles())

	kw.update(&world, 0, timer_info, {})

	expect_value(t, len(world.entities), 2)
	expect_value(t, world.free_tokens.len, 0)
}

@(private = "file")
Sonic :: struct {
	using behavior: kw.Behavior,
	speed:          f64,
}
@(private = "file")
make_sonic :: proc() -> kw.Entity_Payload {
	sonic_behavior := new(Sonic)
	sonic_behavior^ = Sonic {
		speed = 10,
	}

	return kw.Entity_Payload {
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
	using behavior:          kw.Behavior,
	max_flight_duration:     f64,
	get_max_flight_duration: proc(tails: Tails) -> f64,
}
@(private = "file")
make_tails :: proc() -> kw.Entity_Payload {
	tails_behavior := new(Tails)
	tails_behavior^ = Tails {
		max_flight_duration = 9,
		get_max_flight_duration = proc(tails: Tails) -> f64 {
			return tails.max_flight_duration
		},
	}

	return kw.Entity_Payload {
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
	using behavior: kw.Behavior,
	damage:         f64,
}
@(private = "file")
make_knuckles :: proc() -> kw.Entity_Payload {
	knuckles_behavior := new(Knuckles)
	knuckles_behavior^ = Knuckles {
		damage = 8,
	}

	return kw.Entity_Payload {
		tag = "Knuckles",
		transform = kc.Transform_Component {
			position = {80, 80},
			scale = {8, 8},
		},
		behavior = knuckles_behavior,
	}
}

