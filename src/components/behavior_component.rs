use std::{any::Any, fmt::Debug};

use kec::Entity;

pub trait BehaviorComponent: Debug {
    #[allow(unused_variables)]
    #[inline]
    fn on_start(&mut self) {}

    #[allow(unused_variables)]
    #[inline]
    fn on_update(&mut self) {}

    #[allow(unused_variables)]
    #[inline]
    fn on_collision(&mut self, other: &Entity) {}

    #[allow(unused_variables)]
    #[inline]
    fn on_events(&mut self) {}

    #[allow(unused_variables)]
    #[inline]
    fn on_destroy(&mut self) {}

    fn as_any(&self) -> &dyn Any;
}

impl dyn BehaviorComponent {
    #[inline]
    pub fn start(&mut self) {
        self.on_start();
    }

    #[inline]
    pub fn update(&mut self) {
        self.on_update();
    }

    #[inline]
    pub fn notify(&mut self) {
        self.on_events();
    }

    #[inline]
    pub fn destroy(&mut self) {
        self.on_destroy();
    }

    #[inline]
    pub fn collide(&mut self, other: &Entity) {
        self.on_collision(other);
    }
}
