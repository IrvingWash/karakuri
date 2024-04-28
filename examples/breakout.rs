use karakuri::{
    components::{BehaviorComponent, NameComponent, ShapeComponent, TransformComponent},
    math::Vector2,
    utils::{Color, Resolution},
    ComponentsPayload, Engine,
};

fn main() {
    let mut engine = Engine::new(
        String::from("Breakout"),
        Resolution::new(800, 600),
        Color::black(),
        60,
        30,
    );

    let wall_thickness = 10.;
    let resolution = engine.resolution();

    // Left wall
    engine.add_entity(ComponentsPayload {
        name: NameComponent::new(String::from("left wall")),
        shape: Some(ShapeComponent::new(
            Color::white(),
            Vector2::new(wall_thickness, resolution.height as f64),
        )),
        transform: Some(TransformComponent::default().with_position(Vector2::new(
            wall_thickness / 2.,
            resolution.height as f64 / 2.,
        ))),
        behavior: None,
    });

    // Top wall
    engine.add_entity(ComponentsPayload {
        name: NameComponent::new(String::from("top wall")),
        shape: Some(ShapeComponent::new(
            Color::white(),
            Vector2::new(resolution.width as f64, wall_thickness),
        )),
        transform: Some(TransformComponent::default().with_position(Vector2::new(
            resolution.width as f64 / 2.,
            wall_thickness / 2.,
        ))),
        behavior: None,
    });

    // Right wall
    engine.add_entity(ComponentsPayload {
        name: NameComponent::new(String::from("right wall")),
        shape: Some(ShapeComponent::new(
            Color::white(),
            Vector2::new(wall_thickness, resolution.height as f64),
        )),
        transform: Some(TransformComponent::default().with_position(Vector2::new(
            resolution.width as f64 - wall_thickness / 2.,
            resolution.height as f64 / 2.,
        ))),
        behavior: None,
    });

    // Ball
    engine.add_entity(ComponentsPayload {
        name: NameComponent::new(String::from("ball")),
        shape: Some(ShapeComponent::new(Color::white(), Vector2::new(10., 10.))),
        transform: Some(TransformComponent::default().with_position(Vector2::new(
            resolution.width as f64 / 2.,
            resolution.height as f64 / 2.,
        ))),
        behavior: Some(Box::new(BallBehavior::new())),
    });

    // Paddle
    engine.add_entity(ComponentsPayload {
        name: NameComponent::new(String::from("paddle")),
        shape: Some(ShapeComponent::new(Color::white(), Vector2::new(100., 10.))),
        transform: Some(TransformComponent::default().with_position(Vector2::new(
            resolution.width as f64 / 2.,
            resolution.height as f64 - wall_thickness,
        ))),
        behavior: Some(Box::new(PaddleBehavior::new())),
    });

    engine.start();
}

struct PaddleBehavior {
    speed: f64,
    id: Option<usize>,
}

impl PaddleBehavior {
    fn new() -> PaddleBehavior {
        PaddleBehavior {
            speed: 200.,
            id: None,
        }
    }
}

impl BehaviorComponent for PaddleBehavior {
    fn start(&mut self, name_components: &[Option<NameComponent>]) {
        self.id = name_components.iter().position(|name| match name {
            None => false,
            Some(name) => name.value() == "paddle",
        });
    }

    fn update(
        &mut self,
        input_result: &karakuri::InputResult,
        delta_time: f64,
        _spawner: &mut karakuri::Spawner,
        _name_components: &[Option<NameComponent>],
        transform_components: &mut [Option<TransformComponent>],
        _shape_components: &[Option<ShapeComponent>],
    ) {
        let mut velocity = Vector2::new(0., 0.);

        if input_result.a {
            velocity.x = -self.speed;
        }

        if input_result.d {
            velocity.x = self.speed;
        }

        transform_components[self.id.unwrap()]
            .as_mut()
            .unwrap()
            .position
            .add(&velocity.to_scaled(delta_time));
    }
}

struct BallBehavior {
    id: Option<usize>,
    velocity: Vector2,
    paddle_id: Option<usize>,
    left_wall_id: Option<usize>,
    top_wall_id: Option<usize>,
    right_wall_id: Option<usize>,
}

impl BallBehavior {
    pub fn new() -> BallBehavior {
        BallBehavior {
            id: None,
            velocity: Vector2::new(100., 100.),
            paddle_id: None,
            left_wall_id: None,
            top_wall_id: None,
            right_wall_id: None,
        }
    }

    fn collide_with_paddle(
        &mut self,
        position: &Vector2,
        paddle_position: &Vector2,
        paddle_size: &Vector2,
    ) {
        let mut diff = paddle_position.x - position.x;
        if diff <= 0. {
            diff = -diff;
        }

        if diff <= paddle_size.x / 2. && position.y >= paddle_position.y && self.velocity.y > 0. {
            self.velocity.y *= -1.;
        }
    }

    fn collide_with_walls(
        &mut self,
        position: &Vector2,
        top_wall_position: &Vector2,
        left_wall_position: &Vector2,
        right_wall_position: &Vector2,
    ) {
        if position.y <= top_wall_position.y {
            self.velocity.y *= -1.;
        }

        if position.x >= right_wall_position.x || position.x <= left_wall_position.x {
            self.velocity.x *= -1.;
        }
    }
}

impl BehaviorComponent for BallBehavior {
    fn start(&mut self, name_components: &[Option<NameComponent>]) {
        self.id = name_components
            .iter()
            .position(|component| match component {
                None => false,
                Some(component) => component.value() == "ball",
            });

        self.paddle_id = name_components
            .iter()
            .position(|component| match component {
                None => false,
                Some(component) => component.value() == "paddle",
            });

        self.top_wall_id = name_components
            .iter()
            .position(|component| match component {
                None => false,
                Some(component) => component.value() == "top wall",
            });

        self.left_wall_id = name_components
            .iter()
            .position(|component| match component {
                None => false,
                Some(component) => component.value() == "left wall",
            });

        self.right_wall_id = name_components
            .iter()
            .position(|component| match component {
                None => false,
                Some(component) => component.value() == "right wall",
            });
    }

    fn update(
        &mut self,
        _input_result: &karakuri::InputResult,
        delta_time: f64,
        _spawner: &mut karakuri::Spawner,
        _name_components: &[Option<NameComponent>],
        transform_components: &mut [Option<TransformComponent>],
        shape_components: &[Option<ShapeComponent>],
    ) {
        let position = &transform_components[self.id.unwrap()]
            .as_ref()
            .unwrap()
            .position;

        self.collide_with_paddle(
            position,
            &transform_components[self.paddle_id.unwrap()]
                .as_ref()
                .unwrap()
                .position,
            &shape_components[self.paddle_id.unwrap()]
                .as_ref()
                .unwrap()
                .size,
        );

        self.collide_with_walls(
            position,
            &transform_components[self.top_wall_id.unwrap()]
                .as_ref()
                .unwrap()
                .position,
            &transform_components[self.left_wall_id.unwrap()]
                .as_ref()
                .unwrap()
                .position,
            &transform_components[self.right_wall_id.unwrap()]
                .as_ref()
                .unwrap()
                .position,
        );

        transform_components[self.id.unwrap()]
            .as_mut()
            .unwrap()
            .position
            .add(&self.velocity.to_scaled(delta_time));
    }
}
