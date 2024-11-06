use kwindow::Timer;

#[derive(Debug)]
pub struct TimerAdapter<'a> {
    timer: &'a mut Timer,
}

impl<'a> TimerAdapter<'a> {
    pub fn new(timer: &'a mut Timer) -> TimerAdapter<'a> {
        Self { timer }
    }

    pub fn set_interval(&mut self, duration: f64) -> usize {
        self.timer.set_interval(duration)
    }

    pub fn clear_interval(&mut self, id: usize) {
        self.timer.clear_interval(id);
    }

    pub fn set_timeout(&mut self, duration: f64) -> usize {
        self.timer.set_timeout(duration)
    }

    pub fn clear_timeout(&mut self, id: usize) {
        self.timer.clear_timeout(id);
    }
}
