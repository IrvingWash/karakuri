use std::{any::Any, fmt::Debug};

use kec::{Entity, Registry};

use crate::InputProcessorWrapper;

pub struct Ctx<'a> {
    pub delta_time: f64,
    pub registry: &'a Registry,
    pub entity: &'a Entity,
    pub input_processor: &'a InputProcessorWrapper<'a>,
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

    fn as_any(&self) -> &dyn Any;
}
