use std::collections::HashMap;

#[derive(Debug)]
struct Timeout {
    duration: f64,
    start_time: f64,
    callback: fn(),
}

#[derive(Debug, Default)]
pub struct Timer {
    next_id: usize,
    timeouts: HashMap<usize, Timeout>,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            timeouts: HashMap::with_capacity(64),
        }
    }

    pub fn set_timeout(&mut self, callback: fn(), duration: f64) -> usize {
        let id = self.next_id;
        self.next_id += 1;

        self.timeouts.insert(
            id,
            Timeout {
                duration,
                start_time: 0.0,
                callback,
            },
        );

        id
    }

    pub fn clear_timeout(&mut self, id: usize) {
        self.timeouts.remove(&id);
    }

    pub fn update(&mut self, time: f64) {
        let mut timeouts_to_remove: Vec<usize> = Vec::with_capacity(self.timeouts.capacity());

        for (id, timeout) in &self.timeouts {
            if timeout.duration + timeout.start_time >= time {
                (timeout.callback)();

                timeouts_to_remove.push(*id);
            }
        }

        for id in timeouts_to_remove {
            self.timeouts.remove(&id);
        }
    }
}

#[cfg(test)]
mod timer_tests {
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

        let mut player = Player { health: 10 };

        timer.set_timeout(Player::heal, 1000.0);
    }
}
