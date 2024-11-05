use kec::Registry;
use kwindow::Timer;

use crate::{
    components::{BehaviorComponent, Ctx},
    errors::panic_queried,
    Spawner,
};

use super::InputProcessorAdapter;

pub struct TimerAdapter {
    timer: Timer,
}

impl TimerAdapter {
    pub fn new(timer: Timer) -> TimerAdapter {
        Self { timer }
    }

    pub fn set_interval(&mut self, duration: f64) -> usize {
        self.timer.set_interval(duration)
    }

    pub fn clear_interval(&mut self, id: usize) {
        self.timer.clear_interval(id);
    }

    pub fn set_timeout(&mut self, duration: f64) -> usize {
        self.timer.set_timeout(duration)
    }

    pub fn clear_timeout(&mut self, id: usize) {
        self.timer.clear_timeout(id);
    }

    pub fn update(
        &mut self,
        time: f64,
        registry: &mut Registry,
        delta_time: f64,
        input_processor: &InputProcessorAdapter,
        spawner: &mut Spawner,
    ) {
        let finished_timers = self.timer.update(time);

        if finished_timers.is_empty() {
            return;
        }

        let updatable_entities = registry
            .query()
            .with_component::<dyn BehaviorComponent>()
            .build();

        for entity in &updatable_entities {
            let mut behavior = registry
                .get_dyn_component_mut::<dyn BehaviorComponent>(entity)
                .unwrap_or_else(|| panic_queried::<dyn BehaviorComponent>(entity));

            behavior.alarm(
                &finished_timers,
                Ctx {
                    delta_time,
                    entity,
                    input_processor,
                    registry,
                    spawner,
                    timer: self,
                },
            );
        }
    }
}
