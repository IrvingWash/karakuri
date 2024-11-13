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
        f.debug_struct("TimerData")
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
    #[inline]
    pub fn new() -> Self {
        Self {
            time: 0.0,
            next_id: 0,
            timeouts: HashMap::with_capacity(64),
            intervals: HashMap::with_capacity(64),
            finished_timers: HashSet::with_capacity(64),
        }
    }

    #[inline]
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

    #[inline]
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

    #[inline]
    pub fn clear_timeout(&mut self, id: usize) {
        self.timeouts.remove(&id);
    }

    #[inline]
    pub fn clear_interval(&mut self, id: usize) {
        self.intervals.remove(&id);
    }

    #[inline]
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

#[cfg(test)]
mod timer_tests {
    use super::{Timer, TimerData};

    #[test]
    fn test_debug_timer_data() {
        let timer_data = TimerData {
            duration: 15.3,
            start_time: 1.53,
        };

        assert_eq!(
            format!("{:?}", timer_data),
            "TimerData { duration: 15.3, start_time: 1.53 }"
        );
    }

    #[test]
    fn test_new() {
        let timer = Timer::new();

        assert!(timer.timeouts.capacity() > 0);
        assert_eq!(timer.timeouts.len(), 0);
        assert!(timer.intervals.capacity() > 0);
        assert_eq!(timer.intervals.len(), 0);
        assert!(timer.finished_timers.capacity() > 0);
        assert_eq!(timer.finished_timers.len(), 0);
    }

    #[test]
    fn test_set_timeout() {
        let mut timer = Timer::new();

        let timeout_id = timer.set_timeout(5.0);
        let second_timeout_id = timer.set_interval(1.0);
        timer.clear_interval(second_timeout_id);

        assert_eq!(timeout_id, 0);
        assert_eq!(timer.next_id, 2);
        assert_eq!(timer.timeouts.len(), 1);

        timer.consume_finished_timers(1.0);

        assert_eq!(timer.timeouts.get(&timeout_id).unwrap().duration, 5.0);
        assert_eq!(timer.timeouts.get(&timeout_id).unwrap().start_time, 0.0);

        let third_timeout_id = timer.set_timeout(7.0);

        assert_eq!(third_timeout_id, 2);
        assert_eq!(timer.next_id, 3);
        assert_eq!(timer.timeouts.len(), 2);

        assert_eq!(timer.timeouts.get(&third_timeout_id).unwrap().duration, 7.0);
        assert_eq!(
            timer.timeouts.get(&third_timeout_id).unwrap().start_time,
            1.0
        );
    }

    #[test]
    fn test_set_interval() {
        let mut timer = Timer::new();

        let interval_id = timer.set_interval(5.0);
        let second_interval_id = timer.set_interval(1.0);
        timer.clear_interval(second_interval_id);

        assert_eq!(interval_id, 0);
        assert_eq!(timer.next_id, 2);
        assert_eq!(timer.intervals.len(), 1);

        assert_eq!(timer.intervals.get(&interval_id).unwrap().duration, 5.0);
        assert_eq!(timer.intervals.get(&interval_id).unwrap().start_time, 0.0);

        timer.consume_finished_timers(1.0);

        let third_interval_id = timer.set_interval(7.0);

        assert_eq!(third_interval_id, 2);
        assert_eq!(timer.next_id, 3);
        assert_eq!(timer.intervals.len(), 2);

        assert_eq!(
            timer.intervals.get(&third_interval_id).unwrap().duration,
            7.0
        );
        assert_eq!(
            timer.intervals.get(&third_interval_id).unwrap().start_time,
            1.0
        );
    }

    #[test]
    fn test_timer_id_collision() {
        let mut timer = Timer::new();

        let interval_id = timer.set_interval(1.0);
        assert_eq!(interval_id, 0);
        assert_eq!(timer.intervals.len(), 1);
        assert_eq!(timer.timeouts.len(), 0);

        let timeout_id = timer.set_timeout(2.0);
        assert_eq!(timeout_id, 1);
        assert_eq!(timer.intervals.len(), 1);
        assert_eq!(timer.timeouts.len(), 1);
    }

    #[test]
    fn test_timeout_passing() {
        let mut timer = Timer::new();

        let first_timeout_id = timer.set_timeout(5.0);
        let second_timeout_id = timer.set_timeout(10.0);
        let third_timeout_id = timer.set_timeout(10.0);

        timer.clear_timeout(third_timeout_id);

        timer.consume_finished_timers(3.0);
        assert_eq!(timer.timeouts.len(), 2);

        let finished_timers = timer.consume_finished_timers(6.0);

        assert_eq!(timer.timeouts.len(), 1);
        assert!(timer.timeouts.get(&first_timeout_id).is_none());
        assert!(timer.timeouts.get(&second_timeout_id).is_some());

        assert_eq!(finished_timers.len(), 1);
        assert!(finished_timers.get(&first_timeout_id).is_some());
        assert_eq!(
            *finished_timers.get(&first_timeout_id).unwrap(),
            first_timeout_id
        );

        let finished_timers = timer.consume_finished_timers(10.0);

        assert_eq!(timer.timeouts.len(), 0);
        assert!(timer.timeouts.get(&second_timeout_id).is_none());

        assert_eq!(finished_timers.len(), 1);
        assert!(finished_timers.get(&second_timeout_id).is_some());
        assert_eq!(
            *finished_timers.get(&second_timeout_id).unwrap(),
            second_timeout_id
        );

        let fourth_timeout_id = timer.set_timeout(15.0);

        let finished_timers = timer.consume_finished_timers(25.0);

        assert_eq!(timer.timeouts.len(), 0);
        assert!(timer.timeouts.get(&fourth_timeout_id).is_none());

        assert_eq!(finished_timers.len(), 1);
        assert!(finished_timers.get(&fourth_timeout_id).is_some());
        assert_eq!(
            *finished_timers.get(&fourth_timeout_id).unwrap(),
            fourth_timeout_id
        );
    }

    #[test]
    fn test_interval_passing() {
        let mut timer = Timer::new();

        let first_interval = timer.set_interval(5.0);
        let second_interval = timer.set_interval(10.0);
        let third_interval = timer.set_interval(3.0);

        timer.clear_interval(third_interval);

        let finished_timers = timer.consume_finished_timers(3.0);
        assert_eq!(timer.intervals.len(), 2);
        assert_eq!(finished_timers.len(), 0);

        let finished_timers = timer.consume_finished_timers(11.0);
        assert_eq!(finished_timers.len(), 2);
        assert_eq!(
            *finished_timers.get(&first_interval).unwrap(),
            first_interval
        );
        assert_eq!(
            *finished_timers.get(&second_interval).unwrap(),
            second_interval
        );

        let finished_timers = timer.consume_finished_timers(25.0);
        assert_eq!(finished_timers.len(), 2);
        assert_eq!(
            *finished_timers.get(&first_interval).unwrap(),
            first_interval
        );
        assert_eq!(
            *finished_timers.get(&second_interval).unwrap(),
            second_interval
        );
    }
}
