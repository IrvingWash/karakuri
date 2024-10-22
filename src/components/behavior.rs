use std::any::Any;

pub trait Behavior {
    fn start(&mut self) {
        self.on_start();
    }

    fn update(&mut self, delta_time: f64) {
        self.on_update(delta_time);
    }

    fn destroy(&mut self) {
        self.on_destroy();
    }

    fn on_start(&mut self);
    fn on_update(&mut self, delta_time: f64);
    fn on_destroy(&mut self);
    fn as_any(&self) -> &dyn Any;
}
