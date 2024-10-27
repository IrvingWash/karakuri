use karakuri::components::{
    BehaviorComponent, ComponentPayload, SpriteComponent, TagComponent, TransformComponent,
};
use kec::Entity;
use kmath::Vector2;
use kutils::{collision, Color, Size};

pub fn ball_prefab(resolution: &Size) -> ComponentPayload {
    ComponentPayload {
        tag: Some(TagComponent::new(String::from("ball"))),
        sprite: Some(SpriteComponent::new(Size::new(30, 30), Color::WHITE)),
        transform: Some(TransformComponent::from_position(Vector2::new(
            resolution.width as f64 / 2.,
            resolution.height as f64 / 2.,
        ))),
        behavior: Some(Box::new(Ball {
            speed: 400.0,
            resolution: resolution.clone(),
            ..Default::default()
        })),
    }
}

#[derive(Default, Debug)]
struct Ball {
    speed: f64,
    velocity: Vector2,
    left_paddle: Option<Entity>,
    right_paddle: Option<Entity>,
    resolution: Size,
}

impl BehaviorComponent for Ball {
    fn on_start(&mut self, ctx: karakuri::components::Ctx) {
        self.left_paddle = ctx
            .registry
            .find_entity(TagComponent::new(String::from("left-paddle")));
        self.right_paddle = ctx
            .registry
            .find_entity(TagComponent::new(String::from("right-paddle")));

        self.velocity.set(&Vector2::new(self.speed, self.speed));
    }

    fn on_update(&mut self, ctx: karakuri::components::Ctx) {
        let mut transform = ctx
            .registry
            .get_component_mut::<TransformComponent>(&ctx.entity)
            .unwrap();
        let sprite = ctx
            .registry
            .get_component::<SpriteComponent>(&ctx.entity)
            .unwrap();

        let left_paddle_transform = ctx
            .registry
            .get_component::<TransformComponent>(&self.left_paddle.unwrap())
            .unwrap();
        let right_paddle_transform = ctx
            .registry
            .get_component::<TransformComponent>(&self.right_paddle.unwrap())
            .unwrap();
        let paddle_sprite = ctx
            .registry
            .get_component::<SpriteComponent>(&self.left_paddle.unwrap())
            .unwrap();

        if collision::aabb_centered(
            &transform.position,
            &sprite.size,
            &left_paddle_transform.position,
            &paddle_sprite.size,
        ) {
            self.velocity.x *= -1.0;
        }

        if collision::aabb_centered(
            &transform.position,
            &sprite.size,
            &right_paddle_transform.position,
            &paddle_sprite.size,
        ) {
            self.velocity.x *= -1.0;
        }

        if transform.position.y <= 0.0 || transform.position.y >= self.resolution.height as f64 {
            self.velocity.y *= -1.0;
        }

        transform
            .position
            .add(&self.velocity.to_scaled(ctx.delta_time));
    }

    fn on_destroy(&mut self) {}

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
