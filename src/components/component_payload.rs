use super::{Behavior, Sprite, Tag, Transform};

pub struct ComponentPayload {
    pub tag: Option<Tag>,
    pub transform: Option<Transform>,
    pub behavior: Option<Box<dyn Behavior>>,
    pub sprite: Option<Sprite>,
}
