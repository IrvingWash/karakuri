use std::{
    collections::{HashMap, HashSet},
    fmt::{Debug, Formatter, Result},
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

    pub fn is_done(&mut self, id: usize) -> bool {
        self.finished_timers.remove(&id)
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

    pub fn update(&mut self, time: f64) {
        self.time = time;

        self.update_timers(time);
        self.update_intervals(time);
    }

    fn update_timers(&mut self, time: f64) {
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

#[cfg(test)]
mod timer_tests {
    use super::Timer;

    struct Player {
        health: u8,
        healing_id: usize,
    }

    #[test]
    fn test_interval() {
        impl Player {
            fn heal(&mut self) {
                self.health += 1;
            }

            fn start(&mut self, timer: &mut Timer) {
                self.healing_id = timer.set_interval(1000.0);
            }

            fn update(&mut self, timer: &mut Timer) {
                if timer.is_done(self.healing_id) {
                    self.heal();
                }
            }
        }

        let mut timer = Timer::new();

        let mut player = Player {
            healing_id: 0,
            health: 10,
        };

        player.start(&mut timer);

        timer.update(0.0);
        player.update(&mut timer);
        assert_eq!(player.health, 10);

        timer.update(500.0);
        player.update(&mut timer);
        assert_eq!(player.health, 10);

        timer.update(1000.0);
        player.update(&mut timer);
        assert_eq!(player.health, 11);

        timer.update(2000.0);
        player.update(&mut timer);
        assert_eq!(player.health, 12);
    }
}
