use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::{Debug, Formatter, Result},
    rc::Rc,
};

struct TimerData {
    duration: f64,
    start_time: f64,
    callback: Rc<RefCell<dyn FnMut()>>,
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
}

impl Timer {
    pub fn new() -> Self {
        Self {
            time: 0.0,
            next_id: 0,
            timeouts: HashMap::with_capacity(64),
            intervals: HashMap::with_capacity(64),
        }
    }

    pub fn set_timeout(&mut self, callback: Rc<RefCell<dyn FnMut()>>, duration: f64) -> usize {
        let id = self.next_id;
        self.next_id += 1;

        self.timeouts.insert(
            id,
            TimerData {
                duration,
                start_time: self.time,
                callback,
            },
        );

        id
    }

    pub fn set_interval(&mut self, callback: Rc<RefCell<dyn FnMut()>>, duration: f64) -> usize {
        let id = self.next_id;
        self.next_id += 1;

        self.intervals.insert(
            id,
            TimerData {
                duration,
                start_time: self.time,
                callback,
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
                (timeout.callback.borrow_mut())();

                timeouts_to_remove.push(*id);
            }
        }

        for id in timeouts_to_remove {
            self.timeouts.remove(&id);
        }
    }

    fn update_intervals(&mut self, time: f64) {
        for interval in self.intervals.values_mut() {
            if interval.duration + interval.start_time <= time {
                (interval.callback.borrow_mut())();

                interval.start_time = time;
            }
        }
    }
}

#[cfg(test)]
mod timer_tests {
    use std::{cell::RefCell, rc::Rc};

    use super::Timer;

    struct Player {
        health: u8,
    }

    impl Player {
        fn heal(&mut self) {
            self.health += 1;
        }
    }

    #[test]
    fn test_timeouts() {
        let mut timer = Timer::new();

        let player = Rc::new(RefCell::new(Player { health: 10 }));
        let player_clone = Rc::clone(&player);

        let callback = Rc::new(RefCell::new(move || player_clone.borrow_mut().heal()));

        timer.set_timeout(callback, 1000.0);
        assert_eq!(player.borrow().health, 10);

        timer.update(500.0);
        assert_eq!(player.borrow().health, 10);

        timer.update(1000.0);
        assert_eq!(player.borrow().health, 11);
    }

    #[test]
    fn test_intervals() {
        let mut timer = Timer::new();

        let player = Rc::new(RefCell::new(Player { health: 10 }));
        let player_clone = Rc::clone(&player);

        let callback = Rc::new(RefCell::new(move || player_clone.borrow_mut().heal()));

        let id = timer.set_interval(callback, 1000.0);
        assert_eq!(player.borrow().health, 10);

        timer.update(500.0);
        assert_eq!(player.borrow().health, 10);

        timer.update(1000.0);
        assert_eq!(player.borrow().health, 11);

        timer.update(2000.0);
        assert_eq!(player.borrow().health, 12);

        timer.clear_interval(id);

        timer.update(5000.0);
        assert_eq!(player.borrow().health, 12);
    }
}
