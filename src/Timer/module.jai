init :: () {
    timer = .{};
}

deinit :: () {
    for timeout: timer.timeouts {
        if timeout.should_cleanup && timeout.data {
            free(timeout.data);
        }
    }
    for interval: timer.intervals {
        if interval.should_cleanup && interval.data {
            free(interval.data);
        }
    }

    deinit(*timer.timeouts);
    deinit(*timer.intervals);
}

set_timeout :: (duration: float64, proc: (data: $T), data: T, should_cleanup := false) -> u32 {
    defer timer.next_id += 1;

    table_set(
        *timer.timeouts,
        timer.next_id,
        .{
            id             = timer.next_id,
            start_time     = timer.current_time,
            duration       = duration,
            proc           = cast(*void) proc,
            data           = cast(*void) data,
            should_cleanup = should_cleanup,
        }
    );

    return timer.next_id;
}

clear_timeout :: (id: u32) {
    ok, timeout := table_remove(*timer.timeouts, id);
    if ok && timeout.should_cleanup && timeout.data {
        free(timeout.data);
    }
}

set_interval :: (duration: float64, proc: (data: $T), data: T, should_cleanup := false) -> u32
#modify {
    ti := cast(*Type_Info) T;

    return ti.type == .POINTER, "Only pointers can be used as the data type";
}
{
    defer timer.next_id += 1;

    table_set(
        *timer.intervals,
        timer.next_id,
        .{
            id             = timer.next_id,
            start_time     = timer.current_time,
            duration       = duration,
            proc           = cast(*void) proc,
            data           = cast(*void) data,
            should_cleanup = should_cleanup,
        }
    );

    return timer.next_id;
}

clear_interval :: (id: u32) {
    ok, interval := table_remove(*timer.intervals, id);
    if ok && interval.should_cleanup && interval.data {
        free(interval.data);
    }
}

update :: (time: float64) {
    timer.current_time = time;

    timeout_ids_to_delete: [..]u32;
    timeout_ids_to_delete.allocator = temp;
    for *timeout: timer.timeouts {
        if timeout.start_time + timeout.duration <= timer.current_time {
            timeout.proc(timeout.data);

            array_add(*timeout_ids_to_delete, timeout.id);
        }
    }
    for id: timeout_ids_to_delete {
        table_remove(*timer.timeouts, id);
    }

    for *interval: timer.intervals {
        if interval.start_time + interval.duration <= timer.current_time {
            interval.proc(interval.data);

            interval.start_time = timer.current_time;
        }
    }
}

#scope_file

Timer :: struct {
    current_time: float64;
    next_id:      u32;
    timeouts:     Table(u32, Timer_Data);
    intervals:    Table(u32, Timer_Data);
}

Timer_Data :: struct {
    id:             u32;
    start_time:     float64;
    duration:       float64;
    proc:           (data: *void);
    data:           *void;
    should_cleanup: bool;
}

timer: Timer = ---;

#import "Hash_Table";
#import "Basic";
