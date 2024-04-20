use crate::{core::InputResult, scene::Spawner};

use super::{NameComponent, TransformComponent};

pub trait BehaviorComponent {
    fn start(&mut self, name_components: &[Option<NameComponent>]);
    fn update(
        &mut self,
        input_result: &InputResult,
        delta_time: f64,
        spawner: &mut Spawner,
        name_components: &[Option<NameComponent>],
        transform_components: &mut [Option<TransformComponent>],
    );
}
