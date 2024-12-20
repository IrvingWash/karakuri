use std::collections::HashMap;

#[derive(Debug)]
pub struct AnimationControllerComponent {
    current: &'static str,
    animations: HashMap<&'static str, Animation>,
}

impl AnimationControllerComponent {
    #[inline]
    pub fn new(animations: Vec<Animation>) -> Self {
        let current = animations
            .first()
            .expect("At east one animation should be provided.")
            .name;
        let mut animations_map = HashMap::with_capacity(5);

        for animation in animations {
            animations_map.insert(animation.name, animation);
        }

        Self {
            current,
            animations: animations_map,
        }
    }

    #[inline]
    pub fn set_animation(&mut self, name: &'static str) {
        self.current = name;
    }

    #[inline]
    pub fn current(&mut self) -> &mut Animation {
        self.animations.get_mut(self.current).unwrap_or_else(|| {
            klogger::terminate(&format!("Couldn't find animation `{}`", self.current))
        })
    }
}

#[derive(Debug)]
pub struct Animation {
    pub name: &'static str,
    pub texture_name: &'static str,
    pub frame_count: u8,
    pub current_frame: u8,
    pub frame_rate: u8,
    pub looping: bool,
    pub start_time: f64,
}

pub struct AnimationParams {
    pub name: &'static str,
    pub texture_name: &'static str,
    pub frame_count: u8,
    pub frame_rate: u8,
    pub looping: bool,
}

impl Animation {
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub const fn new(params: AnimationParams) -> Self {
        let AnimationParams {
            name,
            texture_name,
            frame_count,
            frame_rate,
            looping,
        } = params;

        Self {
            name,
            texture_name,
            current_frame: 0,
            frame_count,
            frame_rate,
            looping,
            start_time: 0.0,
        }
    }
}

#[cfg(test)]
mod animation_controller_component_tests {
    use super::{Animation, AnimationControllerComponent, AnimationParams};

    #[test]
    fn test_animation_new() {
        let animation = Animation::new(AnimationParams {
            frame_count: 8,
            frame_rate: 10,
            name: "Test",
            texture_name: "Test",
            looping: false,
        });

        assert_eq!(animation.current_frame, 0);
        assert_eq!(animation.start_time, 0.0);
    }

    #[test]
    fn test_controller() {
        let mut controller = AnimationControllerComponent::new(vec![
            Animation::new(AnimationParams {
                name: "Test name",
                texture_name: "Test texture",
                frame_count: 8,
                frame_rate: 10,
                looping: true,
            }),
            Animation::new(AnimationParams {
                frame_count: 10,
                frame_rate: 8,
                looping: false,
                name: "Another test name",
                texture_name: "Another test texture",
            }),
        ]);

        {
            assert_eq!(controller.current().name, "Test name");
            assert_eq!(controller.animations.len(), 2);
        }

        {
            controller.set_animation("Another test name");
            assert_eq!(controller.current().name, "Another test name")
        }
    }
}
