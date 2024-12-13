use super::{
    BehaviorComponent, BoxColliderComponentParams, RigidBodyComponentParams, SpriteComponentParams,
    TagComponentParams, TransformComponentParams,
};

#[derive(Debug)]
pub struct ComponentBundle<'a> {
    pub transform: Option<TransformComponentParams>,
    pub tag: Option<TagComponentParams<'a>>,
    pub behavior: Option<Box<dyn BehaviorComponent>>,
    pub sprite: Option<SpriteComponentParams>,
    pub rigid_body: Option<RigidBodyComponentParams>,
    pub box_collider_component: Option<BoxColliderComponentParams>,
}
