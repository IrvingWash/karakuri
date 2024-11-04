use std::{cell::RefCell, collections::HashMap, fmt::Debug, rc::Weak};

struct Timeout {
    duration: f64,
    start_time: f64,
    callback: Weak<RefCell<dyn FnMut()>>,
}

impl Debug for Timeout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Timeout")
            .field("duration", &self.duration)
            .field("start_time", &self.start_time)
            .finish()
    }
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

    pub fn set_timeout(&mut self, callback: Weak<RefCell<dyn FnMut()>>, duration: f64) -> usize {
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
                if let Some(callback) = timeout.callback.upgrade() {
                    (callback.borrow_mut())()
                }

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

        let callback: Rc<RefCell<dyn FnMut()>> =
            Rc::new(RefCell::new(move || player_clone.borrow_mut().heal()));

        timer.set_timeout(Rc::downgrade(&callback), 1000.0);

        player.borrow_mut().heal();
    }
}
