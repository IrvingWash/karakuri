use karakuri::{
    components::{BehaviorComponent, ComponentPayload, TagComponent},
    EventBundle, UpdateContext,
};
use kmath::Vector2;
use rand::Rng;

use super::enemy_prefab;

pub fn enemy_spawner_prefab() -> ComponentPayload {
    ComponentPayload {
        tag: Some(TagComponent::new(String::from("enemy-spawner"))),
        behavior: Some(Box::new(EnemySpawner::new(1000.0))),
        ..Default::default()
    }
}

#[derive(Debug)]
struct EnemySpawner {
    rate: f64,
    timer_id: i64,
}

impl EnemySpawner {
    fn new(rate: f64) -> Self {
        Self { rate, timer_id: -1 }
    }
}

impl BehaviorComponent for EnemySpawner {
    fn on_start(&mut self, mut ctx: UpdateContext) {
        self.timer_id = ctx.timer.set_interval(self.rate) as i64;
    }

    fn on_events(&mut self, events: &EventBundle, mut ctx: UpdateContext) {
        if events.finished_timers.contains(&(self.timer_id as usize)) {
            ctx.spawner.add_entity(enemy_prefab(Vector2::new(
                800.0 * rand::thread_rng().gen::<f64>(),
                100.0,
            )));
        }

        if events.custom_events.contains("player_died") {
            ctx.timer.clear_interval(self.timer_id as usize);
        }
    }

    fn on_destroy(&mut self, mut ctx: UpdateContext) {
        ctx.timer.clear_interval(self.timer_id as usize);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
