use super::{
    AnimationControllerComponent, BehaviorComponent, BoxColliderComponent, RigidBodyComponent,
    SpriteComponent, TagComponent, TransformComponent,
};

#[derive(Debug, Default)]
pub struct ComponentPayload {
    pub transform: Option<TransformComponent>,
    pub tag: Option<TagComponent>,
    pub behavior: Option<Box<dyn BehaviorComponent>>,
    pub sprite: Option<SpriteComponent>,
    pub animation_controller: Option<AnimationControllerComponent>,
    pub rigid_body: Option<RigidBodyComponent>,
    pub box_collider: Option<BoxColliderComponent>,
}

#[cfg(test)]
mod component_payload_tests {
    use crate::components::{TagComponent, TransformComponent};

    use super::ComponentPayload;

    #[test]
    fn test_default() {
        let component_payload = ComponentPayload {
            tag: Some(TagComponent::new(String::from("Sonic"))),
            transform: Some(TransformComponent::default()),
            ..Default::default()
        };

        assert!(component_payload.tag.is_some());
        assert!(component_payload.transform.is_some());
        assert!(component_payload.behavior.is_none());
        assert!(component_payload.sprite.is_none());
        assert!(component_payload.animation_controller.is_none());
        assert!(component_payload.rigid_body.is_none());
        assert!(component_payload.box_collider.is_none());
    }
}
