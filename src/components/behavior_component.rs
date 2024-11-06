use std::{any::Any, fmt::Debug};

use kec::Entity;

use crate::{
    adapters::{EventSender, InputProcessorAdapter, RegistryAdapter, TimerAdapter},
    event_buss::EventBundle,
    Spawner,
};

pub struct Ctx<'a> {
    pub delta_time: f64,
    pub registry: &'a RegistryAdapter<'a>,
    pub entity: &'a Entity,
    pub input_processor: InputProcessorAdapter<'a>,
    pub spawner: &'a mut Spawner,
    pub timer: TimerAdapter<'a>,
    pub event_sender: EventSender<'a>,
}

pub trait BehaviorComponent: Debug {
    #[allow(unused_variables)]
    fn on_start(&mut self, ctx: Ctx) {}

    #[allow(unused_variables)]
    fn on_update(&mut self, ctx: Ctx) {}

    #[allow(unused_variables)]
    fn on_collision(&mut self, other: &Entity, ctx: Ctx) {}

    #[allow(unused_variables)]
    fn on_events(&mut self, events: &EventBundle, ctx: Ctx) {}

    #[allow(unused_variables)]
    fn on_destroy(&mut self, ctx: Ctx) {}

    fn as_any(&self) -> &dyn Any;
}

impl dyn BehaviorComponent {
    pub fn start(&mut self, ctx: Ctx) {
        self.on_start(ctx);
    }

    pub fn update(&mut self, ctx: Ctx) {
        self.on_update(ctx);
    }

    pub fn notify(&mut self, events: &EventBundle, ctx: Ctx) {
        self.on_events(events, ctx);
    }

    pub fn destroy(&mut self, ctx: Ctx) {
        self.on_destroy(ctx);
    }

    pub fn collide(&mut self, other: &Entity, ctx: Ctx) {
        self.on_collision(other, ctx);
    }
}
