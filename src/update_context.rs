use kec::Entity;

use crate::{
    adapters::{EventSenderAdapter, InputProcessorAdapter, RegistryAdapter, TimerAdapter},
    Spawner,
};

pub struct UpdateContext<'a> {
    pub delta_time: f64,
    pub registry: &'a RegistryAdapter<'a>,
    pub entity: &'a Entity,
    pub input_processor: &'a InputProcessorAdapter<'a>,
    pub spawner: &'a mut Spawner,
    pub timer: &'a mut TimerAdapter<'a>,
    pub event_sender: &'a mut EventSenderAdapter<'a>,
}
