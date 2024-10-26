use std::fmt::Debug;

pub trait BehaviorComponent: Debug {
    fn on_start(&mut self);
    fn on_update(&mut self, delta_time: f64);
    fn on_destroy(&mut self);

    fn start(&mut self) {
        self.on_start();
    }

    fn update(&mut self, delta_time: f64) {
        self.on_update(delta_time);
    }

    fn destroy(&mut self) {
        self.on_destroy();
    }
}
