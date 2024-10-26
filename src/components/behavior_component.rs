use std::fmt::Debug;

pub trait BehaviorComponent: Debug {
    fn on_start(&mut self);
    fn on_update(&mut self);
    fn on_destroy(&mut self);

    fn start(&mut self) {
        self.on_start();
    }

    fn update(&mut self) {
        self.on_update();
    }

    fn destroy(&mut self) {
        self.on_destroy();
    }
}
