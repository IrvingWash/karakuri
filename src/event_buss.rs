use std::{collections::HashSet, mem};

pub enum Event {
    Timer(HashSet<usize>),
    Custom(String),
}

#[derive(Debug, Default)]
pub struct EventBundle {
    pub finished_timers: HashSet<usize>,
    pub custom_events: HashSet<String>,
}

impl EventBundle {
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.finished_timers.is_empty() && self.custom_events.is_empty()
    }
}

#[derive(Debug, Default)]
pub struct EventBuss {
    events: EventBundle,
}

impl EventBuss {
    #[inline]
    pub fn add(&mut self, event: Event) {
        match event {
            Event::Custom(e) => {
                self.events.custom_events.insert(e);
            }
            Event::Timer(e) => {
                self.events.finished_timers = e;
            }
        }
    }

    #[inline]
    pub fn consume_events(&mut self) -> EventBundle {
        mem::take(&mut self.events)
    }
}
