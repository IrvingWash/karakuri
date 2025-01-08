package libs_timer_tests

import "ktimer:timer"
import "core:testing"

@(test)
test_set_timeout :: proc(t: ^testing.T) {
	using testing

	my_timer := timer.new_timer(0)
	defer timer.destroy_timer(my_timer)

	timeout_id := timer.set_timeout(&my_timer, 5)
	second_timeout_id := timer.set_timeout(&my_timer, 1)
	timer.clear_timeout(&my_timer, second_timeout_id)

	expect(t, timeout_id == 0)
	expect(t, my_timer.next_id == 2)
	expect(t, len(my_timer.timeouts) == 1)

	timer.update(&my_timer, 1)

	expect(t, my_timer.timeouts[timeout_id].duration == 5)
	expect(t, my_timer.timeouts[timeout_id].start_time == 0)

	third_timeout_id := timer.set_timeout(&my_timer, 7)

	expect(t, third_timeout_id == 2)
	expect(t, my_timer.next_id == 3)
	expect(t, len(my_timer.timeouts) == 2)

	expect(t, my_timer.timeouts[third_timeout_id].duration == 7)
	expect(t, my_timer.timeouts[third_timeout_id].start_time == 1)
}

@(test)
test_set_interval :: proc(t: ^testing.T) {
	using testing

	my_timer := timer.new_timer(0)
	defer timer.destroy_timer(my_timer)

	interval_id := timer.set_interval(&my_timer, 5)
	second_interval_id := timer.set_interval(&my_timer, 1)
	timer.clear_interval(&my_timer, second_interval_id)

	expect(t, interval_id == 0)
	expect(t, my_timer.next_id == 2)
	expect(t, len(my_timer.intervals) == 1)

	timer.update(&my_timer, 1)

	third_interval_id := timer.set_interval(&my_timer, 7)

	expect(t, third_interval_id == 2)
	expect(t, my_timer.next_id == 3)
	expect(t, len(my_timer.intervals) == 2)

	expect(t, my_timer.intervals[third_interval_id].duration == 7)
	expect(t, my_timer.intervals[third_interval_id].start_time == 1)
}

@(test)
test_timer_id_collision :: proc(t: ^testing.T) {
	using testing

	my_timer := timer.new_timer(0)
	defer timer.destroy_timer(my_timer)

	interval_id := timer.set_interval(&my_timer, 1)
	expect(t, interval_id == 0)
	expect(t, len(my_timer.intervals) == 1)
	expect(t, len(my_timer.timeouts) == 0)

	timeout_id := timer.set_timeout(&my_timer, 2)
	expect(t, timeout_id == 1)
	expect(t, len(my_timer.intervals) == 1)
	expect(t, len(my_timer.timeouts) == 1)
}

@(test)
test_timeout_passing :: proc(t: ^testing.T) {
	using testing

	my_timer := timer.new_timer(0)
	defer timer.destroy_timer(my_timer)

	first_timeout_id := timer.set_timeout(&my_timer, 5)
	second_timeout_id := timer.set_timeout(&my_timer, 10)
	third_timeout_id := timer.set_timeout(&my_timer, 10)

	timer.clear_timeout(&my_timer, third_timeout_id)

	timer.update(&my_timer, 3)
	expect(t, len(my_timer.timeouts) == 2)

	finished_timers := timer.update(&my_timer, 6)
	defer delete(finished_timers)

	expect(t, len(my_timer.timeouts) == 1)
	expect(t, first_timeout_id not_in my_timer.timeouts)
	expect(t, second_timeout_id in my_timer.timeouts)

	expect(t, len(my_timer.finished_timer_ids) == 0)
	expect(t, len(finished_timers) == 1)
	expect(t, first_timeout_id in finished_timers)

	finished_timers_2 := timer.update(&my_timer, 10)
	defer delete(finished_timers_2)

	expect(t, len(my_timer.timeouts) == 0)
	expect(t, second_timeout_id not_in my_timer.timeouts)

	expect(t, len(finished_timers_2) == 1)
	expect(t, second_timeout_id in finished_timers_2)

	fourth_timeout_id := timer.set_timeout(&my_timer, 15)

	finished_timers_3 := timer.update(&my_timer, 25)
	defer delete(finished_timers_3)

	expect(t, len(my_timer.timeouts) == 0)
	expect(t, fourth_timeout_id not_in my_timer.timeouts)

	expect(t, len(finished_timers_3) == 1)
	expect(t, fourth_timeout_id in finished_timers_3)
}

@(test)
test_interval_passing :: proc(t: ^testing.T) {
	using testing

	my_timer := timer.new_timer(0)
	defer timer.destroy_timer(my_timer)

	first_interval := timer.set_interval(&my_timer, 5)
	second_interval := timer.set_interval(&my_timer, 10)
	third_interval := timer.set_interval(&my_timer, 3)

	timer.clear_interval(&my_timer, third_interval)

	finished_timers := timer.update(&my_timer, 3)
	defer delete(finished_timers)
	expect(t, len(my_timer.intervals) == 2)
	expect(t, len(finished_timers) == 0)

	finished_timers_2 := timer.update(&my_timer, 11)
	defer delete(finished_timers_2)
	expect(t, len(finished_timers_2) == 2)
	expect(t, first_interval in finished_timers_2)
	expect(t, second_interval in finished_timers_2)

	finished_timers_3 := timer.update(&my_timer, 25)
	defer delete(finished_timers_3)
	expect(t, len(finished_timers_3) == 2)
	expect(t, first_interval in finished_timers_3)
	expect(t, second_interval in finished_timers_3)
}

