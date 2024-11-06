use crate::event_buss::{Event, EventBuss};

pub enum SendableEvent {
    Custom(String),
}

impl From<SendableEvent> for Event {
    fn from(value: SendableEvent) -> Self {
        match value {
            SendableEvent::Custom(e) => Self::Custom(e),
        }
    }
}

#[derive(Debug)]
pub struct EventSenderAdapter<'a> {
    event_buss: &'a mut EventBuss,
}

impl<'a> EventSenderAdapter<'a> {
    pub fn new(event_buss: &'a mut EventBuss) -> Self {
        Self { event_buss }
    }

    pub fn add(&mut self, event: SendableEvent) {
        self.event_buss.add(event.into());
    }
}
