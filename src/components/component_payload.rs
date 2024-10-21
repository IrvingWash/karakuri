use super::{Tag, Transform};

pub struct ComponentPayload {
    pub tag: Option<Tag>,
    pub transform: Option<Transform>,
}
