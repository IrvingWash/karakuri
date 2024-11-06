use std::{
    collections::{HashMap, HashSet},
    fmt::{Debug, Formatter, Result},
    mem,
};

struct TimerData {
    duration: f64,
    start_time: f64,
}

impl Debug for TimerData {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Timeout")
            .field("duration", &self.duration)
            .field("start_time", &self.start_time)
            .finish()
    }
}

#[derive(Debug, Default)]
pub struct Timer {
    time: f64,
    next_id: usize,
    timeouts: HashMap<usize, TimerData>,
    intervals: HashMap<usize, TimerData>,
    finished_timers: HashSet<usize>,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            time: 0.0,
            next_id: 0,
            timeouts: HashMap::with_capacity(64),
            intervals: HashMap::with_capacity(64),
            finished_timers: HashSet::with_capacity(64),
        }
    }

    pub fn set_timeout(&mut self, duration: f64) -> usize {
        let id = self.next_id;
        self.next_id += 1;

        self.timeouts.insert(
            id,
            TimerData {
                duration,
                start_time: self.time,
            },
        );

        id
    }

    pub fn set_interval(&mut self, duration: f64) -> usize {
        let id = self.next_id;
        self.next_id += 1;

        self.intervals.insert(
            id,
            TimerData {
                duration,
                start_time: self.time,
            },
        );

        id
    }

    pub fn clear_timeout(&mut self, id: usize) {
        self.timeouts.remove(&id);
    }

    pub fn clear_interval(&mut self, id: usize) {
        self.intervals.remove(&id);
    }

    pub fn consume_finished_timers(&mut self, time: f64) -> HashSet<usize> {
        self.time = time;

        self.update_timeouts(time);
        self.update_intervals(time);

        mem::take(&mut self.finished_timers)
    }

    fn update_timeouts(&mut self, time: f64) {
        let mut timeouts_to_remove: Vec<usize> = Vec::with_capacity(self.timeouts.capacity());

        for (id, timeout) in &self.timeouts {
            if timeout.duration + timeout.start_time <= time {
                self.finished_timers.insert(*id);

                timeouts_to_remove.push(*id);
            }
        }

        for id in timeouts_to_remove {
            self.timeouts.remove(&id);
        }
    }

    fn update_intervals(&mut self, time: f64) {
        for (id, interval) in &mut self.intervals {
            if interval.duration + interval.start_time <= time {
                self.finished_timers.insert(*id);

                interval.start_time = time;
            }
        }
    }
}
