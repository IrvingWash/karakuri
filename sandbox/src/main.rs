use karakuri::{
    components::{NameComponent, TransformComponent},
    utils::{Color, Resolution},
    ComponentsPayload, Engine,
};

fn main() {
    let mut engine = Engine::new(
        String::from("Sandbox"),
        Resolution::new(800, 600),
        Color::black(),
        60,
        20,
    );

    engine.add_entity(ComponentsPayload {
        name_component: NameComponent::new(String::from("Joe")),
        transform_component: Some(TransformComponent::default()),
    });

    engine.start();
}
