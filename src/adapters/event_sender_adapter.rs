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

#[cfg(test)]
mod event_sender_adapter_tests {
    use crate::event_buss::{Event, EventBuss};

    use super::{EventSenderAdapter, SendableEvent};

    #[test]
    fn test_sendable_event_from_event() {
        let event = Event::from(SendableEvent::Custom("Test".to_owned()));

        match event {
            Event::Custom(c) => assert_eq!(c, "Test"),
            Event::Timer(_) => assert!(false),
        }
    }

    #[test]
    fn test_event_sender_adapter() {
        let mut event_buss = EventBuss::default();

        let mut event_sender = EventSenderAdapter::new(&mut event_buss);

        event_sender.add(SendableEvent::Custom("Test".into()));

        let events = event_buss.consume_events();

        assert!(events.custom_events.get("Test").is_some());
    }
}
