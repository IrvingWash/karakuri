use karakuri::components::{
    BehaviorComponent, BoxColliderComponent, ComponentPayload, Ctx, SpriteComponent, TagComponent,
    TransformComponent,
};
use karakuri::math::Vector2;
use karakuri::utils::Size;
use karakuri::window::KeyboardKey;

#[derive(PartialEq, Debug)]
pub enum PaddleSide {
    Left,
    Right,
}

impl PaddleSide {
    pub fn to_string(&self) -> &'static str {
        match self {
            Self::Left => "left-paddle",
            Self::Right => "right-paddle",
        }
    }
}

pub fn paddle_prefab(side: PaddleSide, resolution: &Size) -> ComponentPayload {
    ComponentPayload {
        sprite: Some(SpriteComponent {
            texture_name: "square",
            layer: 0,
            ..Default::default()
        }),
        transform: Some(TransformComponent {
            scale: Vector2::new(0.7, 3.0),
            ..Default::default()
        }),
        tag: Some(TagComponent::new(side.to_string().into())),
        behavior: Some(Box::new(Paddle {
            side,
            speed: 30.0,
            resolution: resolution.clone(),
        })),
        box_collider: Some(BoxColliderComponent::default()),
        ..Default::default()
    }
}

#[derive(Debug)]
struct Paddle {
    side: PaddleSide,
    speed: f64,
    resolution: Size,
}

impl BehaviorComponent for Paddle {
    fn on_start(&mut self, ctx: Ctx) {
        let mut transform = ctx
            .registry
            .get_component_mut::<TransformComponent>(&ctx.entity)
            .unwrap();
        let sprite = ctx
            .registry
            .get_component::<SpriteComponent>(&ctx.entity)
            .unwrap();

        let edge_offset = 50.0;

        if self.side == PaddleSide::Left {
            transform.position.set(&Vector2::new(
                edge_offset + sprite.clip_size.as_ref().unwrap().x / 2.0,
                (self.resolution.height / 2) as f64,
            ));
        } else {
            transform.position.set(&Vector2::new(
                (self.resolution.width as f64)
                    - edge_offset
                    - sprite.clip_size.as_ref().unwrap().x / 2.0,
                (self.resolution.height / 2) as f64,
            ));
        }
    }

    fn on_update(&mut self, ctx: Ctx) {
        let mut transform = ctx
            .registry
            .get_component_mut::<TransformComponent>(&ctx.entity)
            .unwrap();

        if self.side == PaddleSide::Left {
            if ctx.input_processor.is_down(KeyboardKey::KEY_W) {
                transform.position.y -= self.speed * ctx.delta_time;
            }
            if ctx.input_processor.is_down(KeyboardKey::KEY_S) {
                transform.position.y += self.speed * ctx.delta_time;
            }
        } else {
            if ctx.input_processor.is_down(KeyboardKey::KEY_UP) {
                transform.position.y -= self.speed * ctx.delta_time;
            }
            if ctx.input_processor.is_down(KeyboardKey::KEY_DOWN) {
                transform.position.y += self.speed * ctx.delta_time;
            }
        }
    }

    fn on_collision(&mut self, other: &kec::Entity, ctx: Ctx) {
        let tag = ctx.registry.get_component::<TagComponent>(other).unwrap();

        println!("{} collided with {}", self.side.to_string(), tag.value())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
