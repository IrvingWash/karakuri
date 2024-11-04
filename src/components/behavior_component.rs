use std::{any::Any, collections::HashSet, fmt::Debug};

use kec::{Entity, Registry};

use crate::{
    adapters::{InputProcessorAdapter, TimerAdapter},
    Spawner,
};

pub struct Ctx<'a> {
    pub delta_time: f64,
    pub registry: &'a Registry,
    pub entity: &'a Entity,
    pub input_processor: &'a InputProcessorAdapter<'a>,
    pub spawner: &'a mut Spawner,
    pub timer: &'a mut TimerAdapter,
}

pub trait BehaviorComponent: Debug {
    #[allow(unused_variables)]
    fn on_start(&mut self, ctx: Ctx) {}
    #[allow(unused_variables)]
    fn on_update(&mut self, ctx: Ctx) {}
    #[allow(unused_variables)]
    fn on_collision(&mut self, other: &Entity, ctx: Ctx) {}
    #[allow(unused_variables)]
    fn on_timer(&mut self, finished_timers: &HashSet<usize>, ctx: Ctx) {}
    #[allow(unused_variables)]
    fn on_destroy(&mut self, ctx: Ctx) {}

    fn start(&mut self, ctx: Ctx) {
        self.on_start(ctx);
    }

    fn update(&mut self, ctx: Ctx) {
        self.on_update(ctx);
    }

    fn alarm(&mut self, finished_timers: &HashSet<usize>, ctx: Ctx) {
        self.on_timer(finished_timers, ctx);
    }

    fn destroy(&mut self, ctx: Ctx) {
        self.on_destroy(ctx);
    }

    fn collide(&mut self, other: &Entity, ctx: Ctx) {
        self.on_collision(other, ctx);
    }

    fn as_any(&self) -> &dyn Any;
}
