use kec::Registry;

use crate::{
    components::{AnimationComponent, SpriteComponent},
    errors::panic_queried,
};

#[derive(Debug, Default)]
pub struct Animator {}

impl Animator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn animate(&self, registry: &mut Registry, time: f64) {
        let animated_entities = registry
            .query()
            .with_component::<AnimationComponent>()
            .with_component::<SpriteComponent>()
            .build();

        for entity in animated_entities {
            let mut animation = registry
                .get_component_mut::<AnimationComponent>(&entity)
                .unwrap_or_else(|| panic_queried::<AnimationComponent>(entity));
            let mut sprite = registry
                .get_component_mut::<SpriteComponent>(&entity)
                .unwrap_or_else(|| panic_queried::<SpriteComponent>(entity));

            animation.current_frame =
                (((time - animation.start_time) * f64::from(animation.frame_rate) / 1000.0)
                    % f64::from(animation.frame_count)) as u8;

            sprite.clip_position.x =
                (i64::from(animation.current_frame) * sprite.clip_size.unwrap().width) as f64;
        }
    }
}
