// @Todo change these to not accept procs as we can't pass anything meaningful inside them

init :: () {
    timer = .{};
}

deinit :: () {
    deinit(*timer.timeouts);
    deinit(*timer.intervals);
}

set_timeout :: (duration: float64, proc: ()) -> u32 {
    defer timer.next_id += 1;

    table_set(
        *timer.timeouts,
        timer.next_id,
        .{
            id = timer.next_id,
            start_time = timer.current_time,
            duration = duration,
            proc = proc,
        }
    );

    return timer.next_id;
}

clear_timeout :: (id: u32) {
    table_remove(*timer.timeouts, id);
}

set_interval :: (duration: float64, proc: ()) -> u32 {
    defer timer.next_id += 1;

    table_set(
        *timer.intervals,
        timer.next_id,
        .{
            id = timer.next_id,
            start_time = timer.current_time,
            duration = duration,
            proc = proc,
        }
    );

    return timer.next_id;
}

clear_interval :: (id: u32) {
    table_remove(*timer.intervals, id);
}

update :: (time: float64) {
    timer.current_time = time;

    timeout_ids_to_delete: [..]u32;
    timeout_ids_to_delete.allocator = temp;
    for *timeout: timer.timeouts {
        if timeout.start_time + timeout.duration <= timer.current_time {
            timeout.proc();

            array_add(*timeout_ids_to_delete, timeout.id);
        }
    }
    for id: timeout_ids_to_delete {
        table_remove(*timer.timeouts, id);
    }

    for *interval: timer.intervals {
        if interval.start_time + interval.duration <= timer.current_time {
            interval.proc();

            interval.start_time = timer.current_time;
        }
    }
}

#scope_file

Timer :: struct {
    current_time: float64;
    next_id: u32;
    timeouts: Table(u32, Timer_Data);
    intervals: Table(u32, Timer_Data);
}

Timer_Data :: struct {
    id: u32;
    start_time: float64;
    duration: float64;
    proc: ();
}

timer: Timer = ---;

#import "Hash_Table";
#import "Basic";
