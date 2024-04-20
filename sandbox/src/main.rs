use karakuri::{
    components::{BehaviorComponent, NameComponent, ShapeComponent, TransformComponent},
    math::Vector2,
    utils::{Color, Resolution},
    ComponentsPayload, Engine, InputResult, Spawner,
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
        name: NameComponent::new(String::from("Joe")),
        transform: Some(TransformComponent::default()),
        shape: Some(ShapeComponent::new(
            Color::white(),
            Vector2::new(100., 100.),
        )),
        behavior: Some(Box::new(Joe::new())),
    });

    engine.start();
}

struct Joe {
    id: Option<usize>,
    speed: Vector2,
}

impl Joe {
    fn new() -> Joe {
        Joe {
            id: None,
            speed: Vector2::new(100., 100.),
        }
    }
}

impl BehaviorComponent for Joe {
    fn start(&mut self, name_components: &[Option<NameComponent>]) {
        self.id = name_components.iter().position(|name| match name {
            None => false,
            Some(name) => name.value() == "Joe",
        });
    }

    fn update(
        &mut self,
        _input_result: &InputResult,
        delta_time: f64,
        _spawner: &mut Spawner,
        _name_components: &[Option<NameComponent>],
        transform_components: &mut [Option<TransformComponent>],
        _shape_components: &[Option<ShapeComponent>],
    ) {
        let id = self.id.unwrap();

        transform_components[id]
            .as_mut()
            .unwrap()
            .position
            .add(&self.speed.to_scaled(delta_time));
    }
}
