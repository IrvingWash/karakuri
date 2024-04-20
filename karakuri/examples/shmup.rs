use karakuri::{
    components::{BehaviorComponent, NameComponent, ShapeComponent, TransformComponent},
    math::Vector2,
    utils::{Color, Resolution},
    ComponentsPayload, Engine,
};

fn main() {
    let mut engine = Engine::new(
        String::from("Shmup"),
        Resolution::new(1920, 1080),
        Color::black(),
        60,
        24,
    );

    engine.add_entity(ComponentsPayload {
        name: NameComponent::new(String::from("ship")),
        transform: Some(
            TransformComponent::default().with_position(Vector2::new(1920. / 2., 980.)),
        ),
        shape: Some(ShapeComponent::new(Color::blue(), Vector2::new(100., 100.))),
        behavior: Some(Box::new(Ship::new())),
    });

    engine.start();
}

struct Ship {
    id: Option<usize>,
    next_projectile_id: u64,
}

impl Ship {
    fn new() -> Ship {
        Ship {
            id: None,
            next_projectile_id: 0,
        }
    }
}

impl BehaviorComponent for Ship {
    fn start(&mut self, name_components: &[Option<NameComponent>]) {
        self.id = name_components.iter().position(|name| match name {
            None => false,
            Some(name) => name.value() == "ship",
        });
    }

    fn update(
        &mut self,
        input_result: &karakuri::InputResult,
        delta_time: f64,
        spawner: &mut karakuri::Spawner,
        _name_components: &[Option<NameComponent>],
        transform_components: &mut [Option<TransformComponent>],
        _shape_components: &[Option<ShapeComponent>],
    ) {
        let transform = transform_components[self.id.unwrap()].as_mut().unwrap();

        if input_result.space {
            let name = String::from(format!("projectile_{}", self.next_projectile_id));

            spawner.add_entity(ComponentsPayload {
                name: NameComponent::new(name.clone()),
                transform: Some(transform.clone()),
                shape: Some(ShapeComponent::new(Color::red(), Vector2::new(10., 10.))),
                behavior: Some(Box::new(Projectile::new(name))),
            });

            self.next_projectile_id += 1;
        }

        let mut velocity = Vector2::new(0., 0.);

        if input_result.a {
            velocity.add(&Vector2::new(-300., 0.));
        }

        if input_result.d {
            velocity.add(&Vector2::new(300., 0.));
        }

        transform.position.add(&velocity.to_scaled(delta_time));
    }
}

struct Projectile {
    id: Option<usize>,
    name: String,
}

impl Projectile {
    fn new(name: String) -> Projectile {
        Projectile { id: None, name }
    }
}

impl BehaviorComponent for Projectile {
    fn start(&mut self, name_components: &[Option<NameComponent>]) {
        self.id = name_components.iter().position(|name| match name {
            None => false,
            Some(name) => name.value() == self.name,
        })
    }

    fn update(
        &mut self,
        _input_result: &karakuri::InputResult,
        delta_time: f64,
        _spawner: &mut karakuri::Spawner,
        _name_components: &[Option<NameComponent>],
        transform_components: &mut [Option<TransformComponent>],
        _shape_components: &[Option<ShapeComponent>],
    ) {
        let velocity = Vector2::new(0., -600.);

        transform_components[self.id.unwrap()]
            .as_mut()
            .unwrap()
            .position
            .add(&velocity.to_scaled(delta_time));
    }
}
