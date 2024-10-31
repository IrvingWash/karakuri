use super::{BehaviorComponent, FigureComponent, TagComponent, TransformComponent};

#[derive(Debug, Default)]
pub struct ComponentPayload {
    pub transform: Option<TransformComponent>,
    pub tag: Option<TagComponent>,
    pub figure: Option<FigureComponent>,
    pub behavior: Option<Box<dyn BehaviorComponent>>,
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
        assert!(component_payload.figure.is_none());
        assert!(component_payload.behavior.is_none());
    }
}
