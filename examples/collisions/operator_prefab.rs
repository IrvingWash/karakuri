use karakuri::{
    components::{BehaviorComponent, CameraComponent, ComponentPayload},
    UpdateContext,
};
use kwindow::KeyboardKey;

pub fn operator_prefab() -> ComponentPayload {
    ComponentPayload {
        camera: Some(CameraComponent {
            zoom: 0.5,
            ..Default::default()
        }),
        behavior: Some(Box::new(Operator {})),
        ..Default::default()
    }
}

#[derive(Debug)]
struct Operator {}

impl BehaviorComponent for Operator {
    fn on_update(&mut self, ctx: UpdateContext) {
        let mut camera = ctx
            .registry
            .get_component_mut::<CameraComponent>(ctx.entity)
            .unwrap();

        if ctx.input_processor.is_down(KeyboardKey::KEY_W) {
            camera.zoom += 0.1 * ctx.delta_time;
        }

        if ctx.input_processor.is_down(KeyboardKey::KEY_S) {
            camera.zoom -= 0.1 * ctx.delta_time;
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
