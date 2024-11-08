use karakuri::components::{BehaviorComponent, CameraComponent, ComponentPayload, TagComponent};
use kwindow::KeyboardKey;

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

    fn on_update(&mut self, ctx: karakuri::UpdateContext) {
        let mut camera = ctx
            .registry
            .get_component_mut::<CameraComponent>(ctx.entity)
            .unwrap();

        if ctx.input_processor.is_down(KeyboardKey::KEY_UP) {
            camera.zoom += 0.1 * ctx.delta_time;
        }

        if ctx.input_processor.is_down(KeyboardKey::KEY_DOWN) {
            camera.zoom -= 0.1 * ctx.delta_time;
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
