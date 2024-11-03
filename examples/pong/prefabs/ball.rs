use karakuri::components::{
    BehaviorComponent, ComponentPayload, FigureComponent, RigidBodyComponent, TagComponent,
    TransformComponent,
};
use karakuri::ec::Entity;
use karakuri::math::Vector2;
use karakuri::utils::{collision, Color, Size};

pub fn ball_prefab(resolution: &Size) -> ComponentPayload {
    ComponentPayload {
        tag: Some(TagComponent::new(String::from("ball"))),
        figure: Some(FigureComponent::new(Size::new(30, 30), Color::WHITE, 0)),
        transform: Some(TransformComponent::from_position(Vector2::new(
            resolution.width as f64 / 2.,
            resolution.height as f64 / 2.,
        ))),
        behavior: Some(Box::new(Ball {
            speed: 30.0,
            resolution: resolution.clone(),
            ..Default::default()
        })),
        rigid_body: Some(RigidBodyComponent::default()),
        ..Default::default()
    }
}

#[derive(Default, Debug)]
struct Ball {
    speed: f64,
    left_paddle: Option<Entity>,
    right_paddle: Option<Entity>,
    resolution: Size,
}

impl BehaviorComponent for Ball {
    fn on_start(&mut self, ctx: karakuri::components::Ctx) {
        self.left_paddle = ctx
            .registry
            .find_entity(&TagComponent::new(String::from("left-paddle")));
        self.right_paddle = ctx
            .registry
            .find_entity(&TagComponent::new(String::from("right-paddle")));

        let mut rigid_body = ctx
            .registry
            .get_component_mut::<RigidBodyComponent>(ctx.entity)
            .unwrap();

        rigid_body
            .velocity
            .set(&Vector2::new(self.speed, self.speed));
    }

    fn on_update(&mut self, ctx: karakuri::components::Ctx) {
        let transform = ctx
            .registry
            .get_component::<TransformComponent>(&ctx.entity)
            .unwrap();
        let figure = ctx
            .registry
            .get_component::<FigureComponent>(&ctx.entity)
            .unwrap();
        let mut rigid_body = ctx
            .registry
            .get_component_mut::<RigidBodyComponent>(ctx.entity)
            .unwrap();

        let left_paddle_transform = ctx
            .registry
            .get_component::<TransformComponent>(&self.left_paddle.unwrap())
            .unwrap();
        let right_paddle_transform = ctx
            .registry
            .get_component::<TransformComponent>(&self.right_paddle.unwrap())
            .unwrap();
        let paddle_figure = ctx
            .registry
            .get_component::<FigureComponent>(&self.left_paddle.unwrap())
            .unwrap();

        if collision::aabb_centered(
            &transform.position,
            &figure.size,
            &left_paddle_transform.position,
            &paddle_figure.size,
        ) {
            rigid_body.velocity.x *= -1.0;
        }

        if collision::aabb_centered(
            &transform.position,
            &figure.size,
            &right_paddle_transform.position,
            &paddle_figure.size,
        ) {
            rigid_body.velocity.x *= -1.0
        }

        if transform.position.y <= 0.0 || transform.position.y >= self.resolution.height as f64 {
            rigid_body.velocity.y *= -1.0;
        }
    }

    fn on_destroy(&mut self) {}

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
