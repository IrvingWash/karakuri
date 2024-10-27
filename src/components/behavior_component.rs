use std::fmt::Debug;

use kec::{Entity, Registry};
use kwindow::InputResult;

pub struct Ctx<'a> {
    pub delta_time: f64,
    pub input_result: &'a InputResult,
    pub registry: &'a Registry,
    pub entity: &'a Entity,
}

pub trait BehaviorComponent: Debug {
    fn on_start(&mut self, ctx: Ctx);
    fn on_update(&mut self, ctx: Ctx);
    fn on_destroy(&mut self);

    fn start(&mut self, ctx: Ctx) {
        self.on_start(ctx);
    }

    fn update(&mut self, ctx: Ctx) {
        self.on_update(ctx);
    }

    fn destroy(&mut self) {
        self.on_destroy();
    }
}
