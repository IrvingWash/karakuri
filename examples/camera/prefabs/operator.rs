use karakuri::components::{BehaviorComponent, CameraComponent, ComponentPayload, TagComponent};

pub fn operator_prefab() -> ComponentPayload {
    ComponentPayload {
        camera: Some(CameraComponent {
            ..Default::default()
        }),
        behavior: Some(Box::new(Operator {})),
        ..Default::default()
    }
}

#[derive(Debug)]
struct Operator {}

impl BehaviorComponent for Operator {
    fn on_start(&mut self, ctx: karakuri::UpdateContext) {
        let mut camera = ctx
            .registry
            .get_component_mut::<CameraComponent>(ctx.entity)
            .unwrap();

        camera.target = ctx
            .registry
            .find_entity(&TagComponent::new(String::from("player")));
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
