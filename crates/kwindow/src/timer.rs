use std::collections::HashMap;

#[derive(Debug)]
struct Interval {
    duration: f64,
    start_time: f64,
}

#[derive(Debug, Default)]
pub struct Timer {
    next_id: usize,
    intervals: HashMap<usize, Interval>,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            intervals: HashMap::with_capacity(64),
        }
    }

    pub fn set_interval(&mut self, duration: f64) -> usize {
        let id = self.next_id;
        self.next_id += 1;

        self.intervals.insert(
            id,
            Interval {
                duration,
                start_time: 0.0,
            },
        );

        id
    }

    pub fn update(&mut self, time: f64) {
        let mut intervals_to_remove: Vec<usize> = Vec::with_capacity(self.intervals.capacity());

        for (id, interval) in &self.intervals {
            if interval.duration + interval.start_time >= time {
                intervals_to_remove.push(*id);
            }
        }

        for id in intervals_to_remove {
            self.intervals.remove(&id);
        }
    }
}
