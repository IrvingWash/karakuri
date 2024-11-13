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

#[cfg(test)]
mod event_buss_tests {
    use std::collections::HashSet;

    use super::{Event, EventBundle, EventBuss};

    #[test]
    fn test_event_bundle_is_empty() {
        let event_bundle = EventBundle::default();

        assert!(event_bundle.is_empty());
    }

    #[test]
    fn test_event_buss() {
        let mut event_buss = EventBuss::default();

        {
            event_buss.add(Event::Custom(String::from("Test")));

            assert!(event_buss.events.finished_timers.is_empty());
            assert!(event_buss.events.custom_events.get("Test").is_some());
        }

        {
            let mut finished_timers = HashSet::new();
            finished_timers.insert(1);

            event_buss.add(Event::Timer(finished_timers));

            assert_eq!(event_buss.events.custom_events.len(), 1);
            assert_eq!(event_buss.events.finished_timers.len(), 1);
        }

        {
            let events = event_buss.consume_events();
            assert!(event_buss.events.is_empty());

            assert!(!events.is_empty());
            assert!(events.custom_events.get("Test").is_some());
            assert!(events.finished_timers.get(&1).is_some());
        }
    }
}
