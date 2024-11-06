use std::{any::Any, fmt::Debug};

use kec::Entity;

use crate::{event_buss::EventBundle, UpdateContext};

pub trait BehaviorComponent: Debug {
    #[allow(unused_variables)]
    fn on_start(&mut self, ctx: UpdateContext) {}

    #[allow(unused_variables)]
    fn on_update(&mut self, ctx: UpdateContext) {}

    #[allow(unused_variables)]
    fn on_collision(&mut self, other: &Entity, ctx: UpdateContext) {}

    #[allow(unused_variables)]
    fn on_events(&mut self, events: &EventBundle, ctx: UpdateContext) {}

    #[allow(unused_variables)]
    fn on_destroy(&mut self, ctx: UpdateContext) {}

    fn as_any(&self) -> &dyn Any;
}

impl dyn BehaviorComponent {
    #[inline]
    pub fn start(&mut self, ctx: UpdateContext) {
        self.on_start(ctx);
    }

    #[inline]
    pub fn update(&mut self, ctx: UpdateContext) {
        self.on_update(ctx);
    }

    #[inline]
    pub fn notify(&mut self, events: &EventBundle, ctx: UpdateContext) {
        self.on_events(events, ctx);
    }

    #[inline]
    pub fn destroy(&mut self, ctx: UpdateContext) {
        self.on_destroy(ctx);
    }

    #[inline]
    pub fn collide(&mut self, other: &Entity, ctx: UpdateContext) {
        self.on_collision(other, ctx);
    }
}
