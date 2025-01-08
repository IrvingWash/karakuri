package ktimer_timer

Timer_Info :: struct {
	time:               f64,
	next_id:            uint,
	timeouts:           map[uint]TimerData,
	intervals:          map[uint]TimerData,
	finished_timer_ids: map[uint]struct {},
}

@(private = "file")
TimerData :: struct {
	duration:   f64,
	start_time: f64,
}

// Creates a new timer
new :: proc(time: f64) -> Timer_Info {
	return Timer_Info {
		time = time,
		next_id = 0,
		timeouts = make(map[uint]TimerData),
		intervals = make(map[uint]TimerData),
		finished_timer_ids = make(map[uint]struct {}),
	}
}

// Destroys a timer
destroy :: proc(timer: Timer_Info) {
	delete(timer.timeouts)
	delete(timer.intervals)
	delete(timer.finished_timer_ids)
}

// Creates a new interval with the given duration
set_interval :: proc(timer: ^Timer_Info, duration: f64) -> uint {
	defer timer.next_id += 1

	timer.intervals[timer.next_id] = TimerData {
		duration   = duration,
		start_time = timer.time,
	}

	return timer.next_id
}

// Creates a new timeout with the given duration
set_timeout :: proc(timer: ^Timer_Info, duration: f64) -> uint {
	defer timer.next_id += 1

	timer.timeouts[timer.next_id] = TimerData {
		duration   = duration,
		start_time = timer.time,
	}

	return timer.next_id
}

// Deletes an interval with the given id
clear_interval :: proc(timer: ^Timer_Info, id: uint) {
	delete_key(&timer.intervals, id)
}

// Deeltes a timeout with the given id
clear_timeout :: proc(timer: ^Timer_Info, id: uint) {
	delete_key(&timer.timeouts, id)
}

// Updates the timer
update :: proc(timer: ^Timer_Info, time: f64) -> map[uint]struct {} {
	timer.time = time

	update_intervals(timer)
	update_timeouts(timer)

	finished_timer_ids := timer.finished_timer_ids
	timer.finished_timer_ids = nil

	return finished_timer_ids
}

@(private)
update_intervals :: proc(timer: ^Timer_Info) {
	for id, &interval in &timer.intervals {
		if interval.duration + interval.start_time < timer.time {
			timer.finished_timer_ids[id] = {}

			interval.start_time = timer.time
		}
	}
}

@(private)
update_timeouts :: proc(timer: ^Timer_Info) {
	timeouts_to_remove := make([dynamic]uint)
	defer delete(timeouts_to_remove)

	for id, timeout in timer.timeouts {
		if timeout.duration + timeout.start_time <= timer.time {
			timer.finished_timer_ids[id] = {}

			append(&timeouts_to_remove, id)
		}
	}

	for id in timeouts_to_remove {
		delete_key(&timer.timeouts, id)
	}
}

