use karakuri::{
    components::{BehaviorComponent, CameraComponent, ComponentPayload, TagComponent},
    EventBundle, UpdateContext,
};
use kwindow::KeyboardKey;

pub fn operator_prefab() -> ComponentPayload {
    ComponentPayload {
        camera: Some(CameraComponent {
            zoom: 0.7,
            ..Default::default()
        }),
        behavior: Some(Box::new(Operator {})),
        ..Default::default()
    }
}

#[derive(Debug)]
struct Operator {}

impl BehaviorComponent for Operator {
    fn on_start(&mut self, ctx: UpdateContext) {
        let mut camera = ctx
            .registry
            .get_component_mut::<CameraComponent>(ctx.entity)
            .unwrap();

        camera.target = ctx
            .registry
            .find_entity(&TagComponent::new(String::from("player")));
    }

    fn on_update(&mut self, ctx: UpdateContext) {
        if ctx.input_processor.is_pressed(KeyboardKey::KEY_O) {
            let mut camera = ctx
                .registry
                .get_component_mut::<CameraComponent>(ctx.entity)
                .unwrap();

            camera.target = None;
            camera.zoom = 1.0;
        }

        if ctx.input_processor.is_pressed(KeyboardKey::KEY_P) {
            let mut camera = ctx
                .registry
                .get_component_mut::<CameraComponent>(ctx.entity)
                .unwrap();

            camera.target = ctx
                .registry
                .find_entity(&TagComponent::new(String::from("player")));
            camera.zoom = 0.7;
        }
    }

    fn on_events(&mut self, events: &EventBundle, ctx: UpdateContext) {
        if events.custom_events.contains("player_died") {
            ctx.registry
                .get_component_mut::<CameraComponent>(ctx.entity)
                .unwrap()
                .target = None;
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
